use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json, Router, body::Body, routing::{post, get},
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::*, query, MySqlPool};
use tracing::{info, trace};

use crate::{state::GlobalState, error::LoginError};

pub fn auth_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/login", post(create_session))
        .route("/logout", post(drop_session))
        .route("/register", post(register))
        .route("/check", get(get_session))
}

#[derive(Serialize, Debug)]
pub struct AuthedUser {
    pub user_id : i32,
    pub company_id : Option<i32>,
    pub mod_id : Option<i32>,
}

impl AuthedUser {
    pub async fn from_token(pool: &MySqlPool, token: &str) -> Result<Self, LoginError> {
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

    pub async fn from_cookie(pool: &MySqlPool, cookies: &CookieJar) -> Result<Self, LoginError> {
        let cookie = cookies.get("token").ok_or(LoginError::AuthError)?;

        let user = Self::from_token(pool, cookie.value()).await?;

        Ok(user)
    }
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
    trace!("Registering user: {} {}", body.email, body.nick);

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
    cookies: CookieJar
) -> Result<impl IntoResponse, LoginError> {
    trace!("Getting session");

    let auth = AuthedUser::from_cookie(&pool, &cookies).await?;

    Ok(Json(json!(auth)))
}

pub async fn drop_session(
    State(pool): State<MySqlPool>,
    cookies: CookieJar
) -> Result<impl IntoResponse, LoginError> {
    let tok = AuthedUser::from_cookie(&pool, &cookies).await?;

    trace!("Drop session: {:?}", tok);

    query!("CALL removeSession( ? )", tok.user_id)
        .execute(&pool)
        .await?;

    trace!("Dropped session: {:?}", tok);

    Ok(StatusCode::OK)
}
