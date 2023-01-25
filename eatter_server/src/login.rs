use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{prelude::*, query, MySqlPool};
use tracing::{error, info, trace};

pub enum LoginError {
    DataBaseError(sqlx::Error),
    HashingError(argon2::password_hash::Error),
    AuthError,
}

pub async fn auth_helper(pool: &MySqlPool, token: String) -> Result<i32, LoginError> {
    trace!("Auth: {:?}", token);
    let res: Option<i32> = query!("CALL getUserFromSession( ? )", token)
        .try_map(|row| row.try_get(0))
        .fetch_optional(pool)
        .await?;

    let res = res.ok_or(LoginError::AuthError)?;

    info!("Retrieved id: {:?}", res);

    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    email: String,
    pass: String,
}

pub async fn create_session(
    State(pool): State<MySqlPool>,
    State(hash_fn): State<Argon2<'static>>,
    Json(body): Json<LoginBody>,
) -> Result<impl IntoResponse, LoginError> {
    trace!("Creating session");

    let mut transaction = pool.begin().await?;

    let hash: String = query!("CALL getPassFromEmail( ? )", body.email)
        .try_map(|row| row.try_get(0))
        .fetch_optional(&mut transaction)
        .await?
        .ok_or(LoginError::AuthError)?;

    let parsed_hash = PasswordHash::new(&hash)?;

    hash_fn.verify_password(body.pass.as_bytes(), &parsed_hash)?;

    let user_id: i32 = query!("CALL getUserIDByEmail( ? )", body.email)
        .try_map(|row| row.try_get(0))
        .fetch_optional(&mut transaction)
        .await?
        .ok_or(LoginError::AuthError)?;

    let session: String = query!("CALL createSession( ? )", user_id)
        .try_map(|row| row.try_get(0))
        .fetch_optional(&mut transaction)
        .await?
        .ok_or(LoginError::AuthError)?;

    transaction.commit().await?;

    trace!("Session created: {}", session);

    Ok(Json(json!({ "token": session })))
}

pub async fn register(
    State(pool): State<MySqlPool>,
    State(hash_fn): State<Argon2<'static>>,
    Json(body): Json<LoginBody>,
) -> Result<impl IntoResponse, LoginError> {
    trace!("Registering user");

    let salt = SaltString::generate(&mut OsRng);

    let hash = hash_fn
        .hash_password(body.pass.as_bytes(), &salt)?
        .to_string();

    query!("CALL addUser( ?, ?, ? )", body.email, body.email, hash)
        .execute(&pool)
        .await
        .map_err(|e| {
            info!("During registration: {:?}", e);
            LoginError::AuthError
        })?;

    trace!("User successfully registered");

    Ok(StatusCode::OK)
}

pub async fn get_session(
    State(pool): State<MySqlPool>,
    Path(tok): Path<String>,
) -> Result<StatusCode, LoginError> {
    trace!("Getting session");

    let _id = auth_helper(&pool, tok).await?;

    Ok(StatusCode::OK)
}

pub async fn drop_session(
    State(pool): State<MySqlPool>,
    Path(tok): Path<String>,
) -> Result<impl IntoResponse, LoginError> {
    trace!("Drop session: {}", tok);

    query!("CALL removeSession( ? )", tok)
        .execute(&pool)
        .await?;

    trace!("Dropped session: {}", tok);

    Ok(StatusCode::OK)
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
