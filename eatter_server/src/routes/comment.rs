use axum::{extract::{State, Path}, response::IntoResponse, Json, Router, body::Body, routing::{get, post, delete}, http::StatusCode};
use axum_extra::extract::CookieJar;
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool, query_as, query};
use tracing::{trace, info};

use crate::{state::GlobalState, error::{ApiError, LoginError}, routes::auth::AuthedUser};

#[derive(Serialize, Debug, FromRow)]
pub struct CommentItem {
    c_id: i32,
    c_body: String,
    c_created_at: NaiveDateTime,
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
        .route("/:id", delete(del_comment))
        .route("/review/:id", get(get_comments_for_review))
}

async fn get_comments_for_review(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
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
) -> Result<impl IntoResponse, ApiError> {
    trace!("Comment to add: {:?}", body);

    let user = AuthedUser::from_cookie(&pool, &cookies).await?;
    
    query!(
        "INSERT INTO comments(body, created_at, review_id, author_id) VALUES (?, ?, ?, ?)",
        body.body,
        Utc::now(),
        body.review_id,
        user.user_id
    )
    .execute(&pool)
    .await?;

    info!("Comment added: {:?}", body);

    Ok(StatusCode::OK)
}

async fn del_comment(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Comment to del: {:?}", id);

    let _mod = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .mod_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "DELETE FROM comments WHERE id = ?", id
    )
    .execute(&pool)
    .await?;

    info!("Comment deleted: {:?}", id);

    Ok(StatusCode::OK)
}