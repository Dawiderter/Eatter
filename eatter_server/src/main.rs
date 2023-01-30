use argon2::Argon2;
use axum::{
    extract::FromRef,
    routing::{get, post, delete},
    Router, http::{StatusCode, Request}, body::Body,
};
use clap::Parser;
use eatter_server::{routes::{auth::auth_router, review::review_router, meal::meal_router, local::local_router, user::user_router, comment::comment_router, search::search_router}};
use eatter_server::state::GlobalState;

use sqlx::{mysql::{MySqlPoolOptions, MySqlConnectOptions}, MySqlPool};
use tracing::{info, warn, trace};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "eatter_server=trace".to_owned()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = MySqlPoolOptions::new()
                .connect(&std::env::var("DATABASE_URL").unwrap())
                .await.unwrap();

    let state = GlobalState { database: pool,  hash_fn: Argon2::default()};

    let app = Router::new()
        .nest("/auth", auth_router())
        .nest("/comment", comment_router())
        .nest("/local" , local_router())
        .nest("/meal", meal_router())
        .nest("/review", review_router())
        .nest("/user", user_router())
        .nest("/search", search_router())
        .fallback(|request : Request<Body>| async move {
            trace!("Request denied: {:?}", request);
            StatusCode::NOT_FOUND
        })
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
