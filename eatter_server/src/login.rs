use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::*, query, MySqlPool, query_as};
use tracing::{error, info, trace};

pub enum LoginError {
    DataBaseError(sqlx::Error),
    HashingError(argon2::password_hash::Error),
    AuthError,
}

#[derive(Serialize, Debug)]
pub struct AuthedUser {
    pub user_id : i32,
    pub company_id : Option<i32>,
    pub mod_id : Option<i32>,
}

pub async fn auth_user(pool: &MySqlPool, token: String) -> Result<AuthedUser, LoginError> {
    trace!("Auth: {:?}", token);

    let user_id = query!("CALL getUserFromSession( ? )", token)
        .try_map(|row| row.try_get(0))
        .fetch_optional(pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    let company_id = query!("SELECT id FROM companies WHERE user_id = ?", user_id)
        .fetch_optional(pool)
        .await?
        .map(|r| r.id);

    let mod_id = query!("SELECT id FROM mods WHERE user_id = ?", user_id)
        .fetch_optional(pool)
        .await?
        .map(|r| r.id);
        
    info!("Retrieved id: {:?}, {:?}, {:?}", user_id, company_id, mod_id);

    Ok(AuthedUser { user_id, company_id, mod_id })
}

pub async fn auth_local_ownership(pool: &MySqlPool, company_id : i32, local_id: i32) -> Result<(), LoginError> {
    trace!("Company local ownership auth: {:?}, {:?}", company_id, local_id);

    let _company_id = query!("SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id WHERE c.id = ? AND l.id = ?", company_id, local_id)
        .fetch_optional(pool)
        .await?
        .ok_or(LoginError::AuthError)?
        .id;

    info!("Authed local ownership");

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct TokenInput {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    email: String,
    pass: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterBody {
    email: String,
    pass: String,
    nick: String,
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
    Json(body): Json<RegisterBody>,
) -> Result<impl IntoResponse, LoginError> {
    trace!("Registering user");

    let salt = SaltString::generate(&mut OsRng);

    let hash = hash_fn
        .hash_password(body.pass.as_bytes(), &salt)?
        .to_string();

    query!("CALL addUser( ?, ?, ? )", body.email, body.nick, hash)
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
    Query(tok): Query<TokenInput>,
) -> Result<impl IntoResponse, LoginError> {
    trace!("Getting session");

    let auth = auth_user(&pool, tok.token).await?;

    Ok(Json(json!(auth)))
}

pub async fn drop_session(
    State(pool): State<MySqlPool>,
    Query(tok): Query<TokenInput>,
) -> Result<impl IntoResponse, LoginError> {
    let tok = tok.token;

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
