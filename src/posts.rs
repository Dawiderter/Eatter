use std::sync::Arc;

use axum::{response::{IntoResponse, Redirect}, extract::State, Form, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_cookies::{Cookies, Cookie};

use crate::state::GlobalState;

#[derive(Debug, Clone)]
pub struct Post {
    pub author : String,
    pub content : String,
    pub comments : Vec<Comment>
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub author : String,
    pub content : String,
}

#[derive(Debug,Deserialize)]
pub struct PostInput {
    pub content: String,
}

pub async fn input_post(cookies: Cookies, State(state) : State<Arc<Mutex<GlobalState>>>, Form(input) : Form<PostInput>) -> impl IntoResponse {
    let cookie = cookies.signed(&state.lock().await.master_key).get("login");
    
    match cookie {
        Some(cookie) =>  {
            state.lock().await.posts.push(Post { author: cookie.value().to_owned(), content: input.content, comments: Vec::new() });
            Redirect::to("/")
        },
        None => {
            Redirect::to("/")
        },
    }
}


