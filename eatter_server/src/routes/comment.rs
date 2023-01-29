use axum::{extract::{State, Path}, response::IntoResponse, Json, Router, body::Body, routing::{get, post}, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool, query_as, query};
use tracing::{trace, info};

use crate::{state::GlobalState, error::{GrabError, PostError}, routes::auth::AuthedUser};

#[derive(Serialize, Debug, FromRow)]
struct CommentItem {
    c_id: i32,
    c_body: String,
    c_created_at: time::PrimitiveDateTime,
    r_id: i32,
    u_id: i32,
    u_nick: String,
}

#[derive(Deserialize, Debug)]
pub struct CommentInput {
    body: String,
    review_id: i32,
}

pub fn comment_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", post(add_comment))
        .route("/review/:id", get(get_comments_for_review))
}

async fn get_comments_for_review(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Comments for review: {:?}", id);

    let res = query_as!(
        CommentItem,
        "SELECT * FROM comment_items WHERE r_id = ?",
        id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(json!(res)))
}

async fn add_comment(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<CommentInput>,
) -> Result<impl IntoResponse, PostError> {
    trace!("Comment to add: {:?}", body);

    let user = AuthedUser::from_cookie(&pool, &cookies).await?;
    
    query!(
        "INSERT INTO comments(body, created_at, review_id, author_id) VALUES (?, ?, ?, ?)",
        body.body,
        time::OffsetDateTime::now_utc(),
        body.review_id,
        user.user_id
    )
    .execute(&pool)
    .await?;

    info!("Comment added: {:?}", body);

    Ok(StatusCode::OK)
}