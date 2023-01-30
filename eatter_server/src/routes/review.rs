use axum::{extract::{State, Path}, response::IntoResponse, Json, Router, body::Body, routing::{get, post}, http::StatusCode};
use axum_extra::extract::CookieJar;
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool, query_as, query};
use tracing::{trace, info};

use crate::{error::ApiError, state::GlobalState, routes::auth::AuthedUser};

#[derive(Serialize, Debug, FromRow)]
struct ReviewItem {
    r_id: i32,
    r_body: String,
    r_created_at: NaiveDateTime,
    r_score: i32,
    u_id: i32,
    u_nick: String,
    m_id: i32,
    m_name: String,
    l_id: i32,
    l_name: String,
}

#[derive(Deserialize, Debug)]
pub struct ReviewInput {
    body: String,
    score: u32,
    meal_id: u32,
}

pub fn review_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", post(add_review))
        .route("/all", get(get_reviews))
        .route("/:id", get(get_review))
        .route("/meal/:id", get(get_reviews_for_meal))
        .route("/followed", get(get_reviews_of_followed))
        .route("/user/:id", get(get_reviews_from_user))
}

async fn get_reviews(
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("All reviews requested");

    let res = query_as!(ReviewItem, "SELECT * FROM feed ORDER BY r_created_at DESC")
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn get_review(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Review: {:?}", id);

    let res = query_as!(ReviewItem, "SELECT * FROM feed WHERE r_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(ApiError::NoItem)?;

    Ok(Json(json!(res)))
}

async fn get_reviews_of_followed(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Personal feed requested",);

    let user = AuthedUser::from_cookie(&pool, &cookies).await?;

    let res = query_as!(ReviewItem, "SELECT * FROM feed WHERE u_id IN (SELECT followed FROM followers WHERE follower = ?) ORDER BY r_created_at DESC", user.user_id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn get_reviews_for_meal(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Reviews for meal: {:?}", id);

    let res = query_as!(ReviewItem, "SELECT * FROM feed WHERE m_id = ? ORDER BY r_created_at DESC", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn get_reviews_from_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Reviews for user: {:?}", id);

    let res = query_as!(ReviewItem, "SELECT * FROM feed WHERE u_id = ? ORDER BY r_created_at DESC", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn add_review(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<ReviewInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Review to add: {:?}", body);

    let user = AuthedUser::from_cookie(&pool, &cookies).await?;

    query!(
        "INSERT INTO reviews(body,created_at,score,meal_id,author_id) VALUES (?, ?, ?, ?, ?)",
        body.body,
        Utc::now(),
        body.score,
        body.meal_id,
        user.user_id
    )
    .execute(&pool)
    .await?;

    info!("Review added: {:?}", body);

    Ok(StatusCode::OK)
}