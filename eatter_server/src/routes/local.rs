use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, FromRow, MySqlPool};
use tracing::{info, trace};

use crate::{
    error::{ApiError, LoginError},
    routes::auth::AuthedUser,
    state::GlobalState,
};

#[derive(Serialize, Debug, FromRow)]
pub struct LocalItem {
    l_id: i32,
    l_name: String,
    l_phone_num: String,
    l_contact_email: String,
    l_address: String,
    c_id: i32,
    c_name: String,
}

#[derive(Deserialize, Debug)]
pub struct LocalInput {
    name: String,
    phone_num: String,
    contact_email: String,
    address: String,
}

pub fn local_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", post(add_local))
        .route("/:id", get(get_local).delete(del_local).patch(patch_local))
        .route("/my", get(get_locals))
}

async fn get_local(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Local: {:?}", id);

    let res = query_as!(LocalItem, "SELECT * FROM local_items WHERE l_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(ApiError::NoItem)?;

    Ok(Json(json!(res)))
}

async fn get_locals(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Company locals");

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    let res = query_as!(
        LocalItem,
        "SELECT * FROM local_items WHERE c_id = ?",
        company_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(json!(res)))
}

async fn add_local(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<LocalInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Local to add: {:?}", body);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!(
        "INSERT INTO locals(name,phone_num,contact_email,address,company_id) VALUES (?, ?, ?, ?, ?)",
        body.name,
        body.phone_num,
        body.contact_email,
        body.address,
        company_id
    )
    .execute(&pool)
    .await?;

    info!("Local added: {:?}", body);

    Ok(StatusCode::OK)
}

async fn del_local(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Local to del: {:?}", id);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!("SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id WHERE c.id = ? AND l.id = ?", company_id,id)
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!("DELETE FROM locals WHERE id = ?", id)
        .execute(&pool)
        .await?;

    info!("Local deleted: {:?}", id);

    Ok(StatusCode::OK)
}

async fn patch_local(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Path(id): Path<i32>,
    Json(body): Json<LocalInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Local to patch: {:?}", id);

    let company_id = AuthedUser::from_cookie(&pool, &cookies)
        .await?
        .company_id
        .ok_or(LoginError::AuthError)?;

    query!("SELECT c.id FROM companies c JOIN locals l ON c.id = l.company_id WHERE c.id = ? AND l.id = ?", company_id,id)
        .fetch_optional(&pool)
        .await?
        .ok_or(LoginError::AuthError)?;

    query!(
        "UPDATE locals SET name = ?, phone_num = ?, contact_email = ?, address = ? WHERE id = ?",
        body.name,
        body.phone_num,
        body.contact_email,
        body.address,
        id
    )
    .execute(&pool)
    .await?;

    info!("Local patched: {:?}", id);

    Ok(StatusCode::OK)
}
