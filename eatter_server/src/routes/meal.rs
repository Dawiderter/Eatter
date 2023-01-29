use axum::{
    body::Body,
    extract::{Path, Query, State},
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
    error::{GrabError, LoginError, PostError},
    routes::auth::AuthedUser,
    state::GlobalState,
};

#[derive(Serialize, Debug, FromRow)]
struct MealItem {
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
        .route("/tag", get(search_meals_by_tag))
}

async fn get_meal(
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

async fn get_meals_from_local(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Meals from local: {:?}", id);

    let res = query_as!(MealItem, "SELECT * FROM meal_items WHERE l_id = ?", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn search_meals_by_tag(
    State(pool): State<MySqlPool>,
    Query(tag): Query<String>,
) -> Result<impl IntoResponse, GrabError> {
    trace!("Tag requested {:?}", tag);

    let res = query_as!(MealItem, "SELECT m.* FROM meal_items m JOIN meals_tags mt ON m.m_id = mt.meal_id JOIN tags t ON t.id = mt.tag_id WHERE t.name LIKE ?", tag)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn add_meal(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<MealInput>,
) -> Result<impl IntoResponse, PostError> {
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
