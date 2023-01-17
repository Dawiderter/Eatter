use std::sync::Arc;

use askama::Template;
use axum::{response::IntoResponse, extract::State};
use tokio::sync::Mutex;
use tower_cookies::Cookies;
use crate::{posts::{Post}, state::GlobalState};

pub async fn base(cookies: Cookies, State(state) : State<Arc<Mutex<GlobalState>>>) -> impl IntoResponse {
    let cookie = cookies.signed(&state.lock().await.master_key).get("login");

    match cookie {
        Some(_) => BaseTemplate { posts : state.lock().await.posts.clone(), logged : true},
        None => BaseTemplate { posts : Vec::new(), logged : false},
    }
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub posts: Vec<Post>,
    pub logged: bool,
}