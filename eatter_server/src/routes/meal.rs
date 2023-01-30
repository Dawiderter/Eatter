use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, FromRow, MySqlPool};
use tracing::{info, trace};

use crate::{
    error::{LoginError, ApiError},
    routes::auth::AuthedUser,
    state::GlobalState,
};

#[derive(Serialize, Debug, FromRow)]
pub struct MealItem {
    m_id: i32,
    m_price: Decimal,
    m_name: String,
    l_id: i32,
    l_name: String,
}

#[derive(Deserialize, Debug)]
pub struct MealInput {
    price: Decimal,
    name: String,
    local_id: i32,
}

pub fn meal_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", post(add_meal))
        .route("/:id", get(get_meal))
        .route("/local/:id", get(get_meals_from_local))
}

async fn get_meal(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Meals: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE m_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(ApiError::NoItem)?;

    Ok(Json(json!(res)))
}

async fn get_meals_from_local(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Meals from local: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE l_id = ?", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn add_meal(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<MealInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Meal to add: {:?}", body);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!("SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id WHERE c.id = ? AND l.id = ?", company_id, body.local_id)
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "INSERT INTO meals(price,name,local_id) VALUES (?, ?, ?)",
        body.price,
        body.name,
        body.local_id
    )
    .execute(&pool)
    .await?;

    info!("Meal added: {:?}", body);

    Ok(StatusCode::OK)
}
