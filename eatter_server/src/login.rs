use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::{extract::State, Form};
use mysql_async::Pool;
use mysql_async::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_cookies::{Cookies, Cookie, Key};
use tracing::info;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct LoginBody{
    email: String,
    pass: String
}

pub async fn create_session(State(pool): State<Pool>, Json(body) : Json<LoginBody>) -> Result<impl IntoResponse, StatusCode> {
    info!("Login: {:?}", body);

    let mut conn = pool.get_conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    conn.exec_drop(
        r"CALL loginUser(:email, :pass, @token)",
                params! {
                    "email" => body.email,
                    "pass" => body.pass,
                }
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res: Option<Option<String>> = conn.query_first("SELECT @token").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(Some(token)) = res {
        Ok((StatusCode::OK, Json(json!({ "token" : token}))))
    }
    else {
        Err(StatusCode::UNAUTHORIZED)
    }

}

pub async fn get_session(State(pool): State<Pool>, Path(tok) : Path<String>) -> Result<StatusCode, StatusCode> {

    info!("Auth: {:?}", tok);

    let mut conn = pool.get_conn().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    if tok == "hej" {
        Ok(StatusCode::OK)
    }
    else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
 