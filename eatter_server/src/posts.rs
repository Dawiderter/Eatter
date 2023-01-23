use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mysql_async::{prelude::*, Pool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::login::auth_helper;

#[derive(Deserialize, Debug)]
pub struct Review {
    body: String,
    score: u32,
    meal_id: u32,
}

pub async fn add_review(
    State(pool): State<Pool>,
    Path(token): Path<String>,
    Json(body): Json<Review>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Review to add: {:?}", body);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = auth_helper(&mut conn, token)
        .await?;

    conn.exec_drop(
        r"CALL addReview(:body, :score, :meal_id, :author_id)",
        params! {
            "body" => body.body,
            "score" => body.score,
            "meal_id" => body.meal_id,
            "author_id" => user,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
