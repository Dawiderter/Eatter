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
    m_r_num: i32,
    m_r_avg: Option<Decimal>,
}

#[derive(Serialize, Debug, FromRow)]
pub struct Tag {
    name: String
}

#[derive(Deserialize, Debug)]
pub struct MealInput {
    price: Decimal,
    name: String,
    local_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct TagInput {
    meal_id: i32,
    tag_name: String,
}

pub fn meal_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", post(add_meal))
        .route("/:id", get(get_meal).delete(del_meal).patch(patch_meal))
        .route("/local/:id", get(get_meals_from_local))
        .route("/tags/:id", get(get_tags))
        .route("/tag", post(add_tag).delete(del_tag))
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

async fn add_tag(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<TagInput>,
) -> Result<impl IntoResponse, ApiError> { 
    trace!("Tag to add: {:?}", body);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id JOIN meals m ON l.id = m.local_id WHERE c.id = ? AND m.id = ?",
        company_id, 
        body.meal_id
    )
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "CALL addTagForMeal(?, ?)",
        body.tag_name,
        body.meal_id
    )
        .execute(&pool)
        .await?;

    info!("Tag added: {:?}", body);

    Ok(StatusCode::OK)
}

async fn get_tags(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Tags for meal: {:?}", id);

    let res = query_as!(Tag, "SELECT t.name FROM tags t JOIN meals_tags tm ON t.id = tm.tag_id JOIN meals m ON tm.meal_id = m.id WHERE m.id = ?", id)    
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn del_meal(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Meal to del: {:?}", id);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id JOIN meals m ON l.id = m.local_id WHERE c.id = ? AND m.id = ?",
        company_id, 
        id
    )
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "DELETE FROM meals WHERE id = ?", id
    )
        .execute(&pool)
        .await?;

    info!("Meal deleted: {:?}", id);

    Ok(StatusCode::OK)
}

async fn del_tag(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<TagInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Tag to del: {:?}", body);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id JOIN meals m ON l.id = m.local_id WHERE c.id = ? AND m.id = ?",
        company_id, 
        body.meal_id
    )
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "DELETE mt FROM meals_tags mt JOIN tags t ON t.id = mt.tag_id WHERE t.name = ? AND mt.meal_id = ?", body.tag_name, body.meal_id
    )
        .execute(&pool)
        .await?;

    info!("Tag deleted: {:?}", body);

    Ok(StatusCode::OK)
}

async fn patch_meal(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Path(id): Path<i32>,
    Json(body): Json<MealInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Local to patch: {:?}", id);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id JOIN meals m ON l.id = m.local_id WHERE c.id = ? AND m.id = ?",
        company_id, 
        id
    )
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "UPDATE meals SET price = ?, name = ? WHERE id = ?",
        body.price,
        body.name,
        id
    )
    .execute(&pool)
    .await?;

    info!("Local patched: {:?}", id);

    Ok(StatusCode::OK)
}