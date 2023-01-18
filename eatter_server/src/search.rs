use axum::response::{IntoResponse, Redirect};

pub async fn search() -> impl IntoResponse {
    Redirect::to("/")
}