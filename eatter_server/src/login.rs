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

    conn.exec_iter(
        r"INSERT INTO sessions (session, user_id) 
                VALUES (:session, :user_id)",
                params! {
                    "session" => "test",
                    "user_id" => 1,
                }
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;



    Ok((StatusCode::OK, Json(json!({ "token" : body.email}))))
}

pub async fn get_session(State(pool): State<Pool>, Path(tok) : Path<String>) -> impl IntoResponse {

    info!("Auth: {:?}", tok);

    if tok == "hej" {
        StatusCode::OK
    }
    else {
        StatusCode::UNAUTHORIZED
    }
}
 