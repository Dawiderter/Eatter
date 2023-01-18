use std::sync::Arc;

use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_cookies::{Cookie, Cookies, Key};

use crate::{db::Database, state::GlobalState};

#[derive(Debug, Clone)]
pub struct Post {
    pub post_id: usize,
    pub author: String,
    pub content: String,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub author: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct PostInput {
    pub content: String,
}

pub async fn input_post(
    cookies: Cookies,
    State(key): State<Key>,
    State(database): State<Database>,
    Form(input): Form<PostInput>,
) -> impl IntoResponse {
    let cookie = cookies.signed(&key).get("login");
    match cookie {
        Some(cookie) => {
            let id = database
                .create_post(cookie.value().to_owned(), input.content)
                .await;
            Redirect::to(&format!("/post/{id}"))
        }
        None => Redirect::to("/"),
    }
}


#[derive(Debug, Deserialize)]
pub struct CommentInput {
    pub content: String,
}

pub async fn input_comment(
    cookies: Cookies,
    State(key): State<Key>,
    State(database): State<Database>,
    Path(post_id): Path<usize>,
    Form(input): Form<CommentInput>,
) -> impl IntoResponse {
    let cookie = cookies.signed(&key).get("login");
    match cookie {
        Some(cookie) => {
            let id = database
                .create_comment(post_id, cookie.value().into(), input.content)
                .await;
            Redirect::to("/")
        }
        None => Redirect::to("/"),
    }
}