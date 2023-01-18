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


pub async fn login(cookies: Cookies, State(key) : State<Key>, Form(form) : Form<LoginForm>) -> impl IntoResponse {
    cookies.signed(&key).add(Cookie::new("login", form.email));

    Redirect::to("/")
}

 