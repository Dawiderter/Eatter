use axum::{extract::{State, Path}, response::IntoResponse, Json, Router, body::Body, routing::{get, post}, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool, query_as, query};
use tracing::{trace, info};

use crate::{error::ApiError, state::GlobalState, routes::auth::AuthedUser};

#[derive(Serialize, Debug, FromRow)]
struct UserItem {
    u_id: i32,
    u_nick: String,
    u_bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FollowInput {
    follow: i32,
}

#[derive(Deserialize,Debug)]
pub struct BioInput{
    bio: String,
}

pub fn user_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/follow", post(follow))
        .route("/unfollow", post(unfollow))
        .route("/bio", post(change_bio))
        .route("/:id", get(get_user))
        .route("/followers/:id", get(get_user_followers))
        .route("/followed/:id", get(get_user_followed))
}

async fn get_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("User: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id = ?", id)
        .fetch_optional(&pool)
        .await?
        .ok_or(ApiError::NoItem)?;

    Ok(Json(json!(res)))
}

async fn get_user_followers(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Followers for user: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id IN (SELECT follower FROM followers WHERE followed = ?)", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn get_user_followed(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Followed for user: {:?}", id);

    let res = query_as!(UserItem, "SELECT * FROM user_items WHERE u_id IN (SELECT followed FROM followers WHERE follower = ?)", id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(json!(res)))
}

async fn change_bio(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<BioInput>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Bio to change: {:?}", body);

    let user_id = AuthedUser::from_cookie(&pool, &cookies).await?.user_id;

    query!(
        "UPDATE users_ext SET bio = ? WHERE id = ?",
        body.bio,
        user_id
    )
    .execute(&pool)
    .await?;

    info!("Bio changed: {:?}", body);

    Ok(StatusCode::OK)
}

async fn follow(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<FollowInput>,
) -> Result<impl IntoResponse, ApiError> {
    info!("User to follow: {:?}", body);

    let user_id = AuthedUser::from_cookie(&pool, &cookies).await?.user_id;

    query!(
        "INSERT INTO followers(follower, followed) VALUES (? , ?)",
        user_id,
        body.follow,
    )
    .execute(&pool)
    .await?;

    info!("User followed: {:?} {:?}", user_id, body.follow);

    Ok(StatusCode::OK)
}

async fn unfollow(
    State(pool): State<MySqlPool>,
    cookies: CookieJar,
    Json(body): Json<FollowInput>,
) -> Result<impl IntoResponse, ApiError> {
    info!("User to unfollow: {:?}", body);

    let user_id = AuthedUser::from_cookie(&pool, &cookies).await?.user_id;

    query!(
        "DELETE FROM followers WHERE followed = ? AND follower = ?",
        body.follow,
        user_id,
    )
    .execute(&pool)
    .await?;

    info!("User unfollowed: {:?} {:?}", user_id, body.follow);

    Ok(StatusCode::OK)
}