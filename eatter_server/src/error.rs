use axum::{response::IntoResponse, http::StatusCode};
use tracing::{error, info};

pub enum LoginError {
    DataBaseError(sqlx::Error),
    HashingError(argon2::password_hash::Error),
    AuthError,
}

pub enum GrabError {
    DataBaseError(sqlx::Error),
    NoItem,
}

pub enum PostError {
    DataBaseError(sqlx::Error),
    LoginError(LoginError),
}

impl From<sqlx::Error> for GrabError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl IntoResponse for GrabError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NoItem => {
                info!("No item");
                StatusCode::NOT_FOUND
            }
        }
        .into_response()
    }
}

impl From<sqlx::Error> for LoginError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl From<argon2::password_hash::Error> for LoginError {
    fn from(inner: argon2::password_hash::Error) -> Self {
        match inner {
            argon2::password_hash::Error::Password => Self::AuthError,
            _ => Self::HashingError(inner),
        }
    }
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        match self {
            LoginError::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginError::HashingError(inner) => {
                error!("Hashing error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginError::AuthError => {
                info!("Unauthorized access");
                StatusCode::UNAUTHORIZED
            }
        }
        .into_response()
    }
}

impl From<sqlx::Error> for PostError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl From<LoginError> for PostError {
    fn from(inner: LoginError) -> Self {
        Self::LoginError(inner)
    }
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::LoginError(inner) => {
                error!("Post login error");
                inner.into_response()
            }
        }
    }
}
