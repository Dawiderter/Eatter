use argon2::Argon2;
use axum::{
    extract::FromRef,
    routing::{get, post, delete},
    Router,
};
use clap::Parser;
use eatter_server::{login, grab, search};

use sqlx::{Pool, mysql::{MySqlPoolOptions, MySqlConnectOptions}, MySqlPool};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub database: MySqlPool,
    pub hash_fn: Argon2<'static>,
}

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
        //.route("/search", get(search::search))
        .route("/login", post(login::create_session))
        .route("/logout/:tok", delete(login::drop_session))
        .route("/register", post(login::register))
        .route("/auth/:tok", get(login::get_session))
        //.route("/grab/local/:id/meals", get(grab::get_meals_from_local))
        //.route("/grab/meal/:id/reviews", get(grab::get_reviews_from_meal))
        //.route("/grab/review/:id", get(grab::get_review))
        .route("/grab/feed/global", get(grab::get_global_feed))
        //.route("/post/review/:tok", post(posts::add_review))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
