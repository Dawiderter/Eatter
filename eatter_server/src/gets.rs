use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Serialize;
use serde_json::json;
use sqlx::{query, query_as, FromRow, MySqlPool};
use tracing::{error, info, trace};

pub enum GrabError {
    DataBaseError(sqlx::Error),
    NoItem,
}

#[derive(Serialize, Debug, FromRow)]
struct MealItem {
    m_id: i32,
    m_price: Decimal,
    m_name: String,
    l_id: i32,
    l_name: String,
}

#[derive(Serialize, Debug, FromRow)]
struct FeedItem {
    r_id: i32,
    r_body: String,
    r_created_at: time::PrimitiveDateTime,
    r_score: i32,
    u_id: i32,
    u_nick: String,
    m_id: i32,
    m_name: String,
    l_id: i32,
    l_name: String,
}

#[derive(Serialize, Debug, FromRow)]
struct CommentItem {
    c_id: i32,
    c_body: String,
    c_created_at: time::PrimitiveDateTime,
    r_id: i32,
    u_id: i32,
    u_nick: String,
}

#[derive(Serialize, Debug, FromRow)]
struct UserItem {
    u_id: i32,
    u_nick: String,
    u_bio: Option<String>,
}

pub async fn get_meals_from_local(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Meals from local: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE l_id = ?", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_meal(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Meals: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE m_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(GrabError::NoItem)?;

    Ok(Json(json!(res)))
}

pub async fn get_reviews_for_meal(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Reviews for meal: {:?}", id);

    let res = query_as!(FeedItem, "SELECT * FROM feed WHERE m_id = ?", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_comments_for_review(
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

pub async fn get_feed_item(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Review: {:?}", id);

    let res = query_as!(FeedItem, "SELECT * FROM feed WHERE r_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(GrabError::NoItem)?;

    Ok(Json(json!(res)))
}

pub async fn get_user_item(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("User: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(GrabError::NoItem)?;

    Ok(Json(json!(res)))
}

pub async fn get_user_followers(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Followers for user: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id IN (SELECT follower FROM followers WHERE followed = ?)", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_user_followed(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Followed for user: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id IN (SELECT followed FROM followers WHERE follower = ?)", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_global_feed(
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Global feed requested");

    let res = query_as!(FeedItem, "SELECT * FROM feed ORDER BY r_created_at DESC")
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn search_meals_by_tag(
    State(pool): State<MySqlPool>,
    Query(tag): Query<String>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Tag requested {:?}", tag);

    let res = query_as!(MealItem, "SELECT m.* FROM meal_items m JOIN meals_tags mt ON m.m_id = mt.meal_id JOIN tags t ON t.id = mt.tag_id WHERE t.name LIKE ?", tag)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

impl From<sqlx::Error> for GrabError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl IntoResponse for GrabError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NoItem => {
                info!("No item");
                StatusCode::NOT_FOUND
            }
        }
        .into_response()
    }
}
