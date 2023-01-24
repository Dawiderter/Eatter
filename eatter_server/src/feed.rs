use std::sync::Arc;

use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use mysql_async::Pool;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_cookies::{Cookie, Cookies, Key};

// pub async fn get_globfeed(State(pool): State<Pool>, Path(tok) : Path<String>) -> Result<StatusCode, StatusCode> {
//     if let Some(Some(token)) = res {
//         Ok((StatusCode::OK, Json(json!({ "token" : token}))))
//     }
//     else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }