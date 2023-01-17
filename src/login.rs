use std::sync::Arc;

use axum::response::{IntoResponse, Redirect};
use axum::{extract::State, Form};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_cookies::{Cookies, Cookie, Key};

use crate::state::GlobalState;

#[derive(Debug, Deserialize)]
pub struct LoginForm{
    email: String,
    pass: String
}


pub async fn login(cookies: Cookies, State(state) : State<Arc<Mutex<GlobalState>>>, Form(form) : Form<LoginForm>) -> impl IntoResponse {
    cookies.signed(&state.lock().await.master_key).add(Cookie::new("login", form.email));

    Redirect::to("/")
}