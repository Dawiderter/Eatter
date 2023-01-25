use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, MySqlPool};
use tracing::{error, info, trace};

use crate::login::{auth_user, LoginError, auth_company, auth_local_ownership, TokenInput};

pub enum PostError {
    DataBaseError(sqlx::Error),
    LoginError(LoginError),
}

#[derive(Deserialize, Debug)]
pub struct ReviewInput {
    body: String,
    score: u32,
    meal_id: u32,
}

pub async fn add_review(
    State(pool): State<MySqlPool>,
    Query(tok): Query<TokenInput>,
    Json(body): Json<ReviewInput>,
) -> Result<impl IntoResponse, PostError> {
    let token = tok.token;

    trace!("Review to add: {:?}", body);

    let user = auth_user(&pool, token).await?;

    query!(
        "INSERT INTO reviews(body,created_at,score,meal_id,author_id) VALUES (?, ?, ?, ?, ?)",
        body.body,
        time::OffsetDateTime::now_utc(),
        body.score,
        body.meal_id,
        user
    )
    .execute(&pool)
    .await?;

    info!("Review added: {:?}", body);

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
pub struct CommentInput {
    body: String,
    review_id: i32,
}

pub async fn add_comment(
    State(pool): State<MySqlPool>,
    Query(tok): Query<TokenInput>,
    Json(body): Json<CommentInput>,
) -> Result<impl IntoResponse, PostError> {
    let token = tok.token;

    trace!("Comment to add: {:?}", body);

    let user = auth_user(&pool, token).await?;

    query!(
        "INSERT INTO comments(body, created_at, review_id, author_id) VALUES (?, ?, ?, ?)",
        body.body,
        time::OffsetDateTime::now_utc(),
        body.review_id,
        user
    )
    .execute(&pool)
    .await?;

    info!("Comment added: {:?}", body);

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
pub struct LocalInput {
    name: String,
    phone_num: String,
    contact_email: String,
    address: String,
}

pub async fn add_local(
    State(pool): State<MySqlPool>,
    Query(tok): Query<TokenInput>,
    Json(body): Json<LocalInput>,
) -> Result<impl IntoResponse, PostError> {
    let token = tok.token;

    trace!("Local to add: {:?}", body);

    let company_id = auth_company(&pool, token).await?;

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

#[derive(Deserialize, Debug)]
pub struct MealInput {
    price: Decimal,
    name: String,
    local_id: i32,
}

pub async fn add_meal(
    State(pool): State<MySqlPool>,
    Query(tok): Query<TokenInput>,
    Json(body): Json<MealInput>,
) -> Result<impl IntoResponse, PostError> {
    let token = tok.token;

    trace!("Meal to add: {:?}", body);

    let _company_id = auth_local_ownership(&pool, token, body.local_id).await?;

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

impl From<sqlx::Error> for PostError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl From<LoginError> for PostError {
    fn from(inner: LoginError) -> Self {
        Self::LoginError(inner)
    }
}

impl IntoResponse for PostError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::LoginError(inner) => {
                error!("Post login error");
                inner.into_response()
            }
        }
    }
}
