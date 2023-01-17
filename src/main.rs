
use std::sync::Arc;

use axum::{
    routing::{get, get_service, post},
    Router, http::StatusCode,
};
use eatter::{base, state::GlobalState, login, search, posts};
use tokio::sync::Mutex;
use tower_cookies::{Key, CookieManagerLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let key = Key::generate();
    let state = Arc::new(Mutex::new(GlobalState {master_key : key.clone(), posts : Vec::new()}));

    let app = Router::new()
        .route("/", get(base::base))
        .route("/search", get(search::search))
        .route("/post", post(posts::input_post))
        .route("/login", post(login::login))
        .nest_service("/assets", get_service(ServeDir::new("assets")).handle_error(|err| async {(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")} ))
        .layer(CookieManagerLayer::new())
        .with_state(state);
        
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
