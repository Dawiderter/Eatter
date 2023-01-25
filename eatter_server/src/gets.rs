use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Serialize;
use serde_json::json;
use sqlx::{MySqlPool, query_as};
use tracing::{info, error, trace};

pub enum GrabError {
    DataBaseError(sqlx::Error),
    NoItem,
}

#[derive(Serialize, Debug)]
struct MealItem {
    m_id: i32,
    m_price: Decimal,
    m_name: String,
    l_id: i32,
    l_name: String,
}

#[derive(Serialize, Debug)]
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

pub async fn get_meals_from_local(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    info!("Meals from local: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE l_id = ?", id).fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_reviews_for_meal(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    info!("Reviews from meal: {:?}", id);

    let res = query_as!(FeedItem, "SELECT * FROM feed WHERE m_id = ?", id).fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

pub async fn get_feed_item(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    info!("Review: {:?}", id);

    let res = query_as!(FeedItem, "SELECT * FROM feed WHERE r_id = ?", id).fetch_optional(&pool)
        .await?
        .ok_or(GrabError::NoItem)?;

    Ok(Json(json!(res)))
}

pub async fn get_global_feed(State(pool): State<MySqlPool>) -> Result<impl IntoResponse, GrabError> {
    trace!("Global feed requested");

    let res = query_as!(FeedItem, "SELECT * FROM feed").fetch_all(&pool).await?;

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