use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use mysql_async::prelude::*;
use mysql_async::{Conn, Pool};
use serde::Deserialize;
use serde_json::json;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    email: String,
    pass: String,
}

pub async fn auth_helper(conn: &mut Conn, token: String) -> Result<u32, StatusCode> {
    let res: Option<Option<u32>> = conn
        .exec_first(
            r"CALL getUserFromSession(:token)",
            params! {
                "token" => token,
            },
        )
        .await
        .map_err(|_| {
            info!("Wrong auth");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(Some(id)) = res {
        info!("Logged user {id}");
        Ok(id)
    } else {
        info!("Wrong auth");
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn create_session(
    State(pool): State<Pool>,
    Json(body): Json<LoginBody>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Login: {:?}", body);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res: Option<Option<String>> = conn
        .exec_first(
            r"CALL loginUser(:email, :pass)",
            params! {
                "email" => body.email,
                "pass" => body.pass,
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //let res: Option<Option<String>> = conn.query_first("SELECT @token").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(Some(token)) = res {
        Ok((StatusCode::OK, Json(json!({ "token": token }))))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn get_session(
    State(pool): State<Pool>,
    Path(tok): Path<String>,
) -> Result<StatusCode, StatusCode> {
    info!("Auth: {:?}", tok);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = auth_helper(&mut conn, tok).await?;

    info!("Retrieved id: {:?}", id);

    Ok(StatusCode::OK)
}

pub async fn drop_session(
    State(pool): State<Pool>,
    Path(tok): Path<String>,
) -> Result<StatusCode, StatusCode> {
    info!("Drop session: {:?}", tok);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    conn.exec_drop(
        r"CALL removeSession(:token)",
        params! {
            "token" => tok,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
