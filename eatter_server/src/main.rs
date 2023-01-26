use argon2::Argon2;
use axum::{
    extract::FromRef,
    routing::{get, post, delete},
    Router,
};
use clap::Parser;
use eatter_server::{login, gets, search, posts};

use sqlx::{Pool, mysql::{MySqlPoolOptions, MySqlConnectOptions}, MySqlPool};
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
        .route("/login", post(login::create_session))
        .route("/logout", delete(login::drop_session))
        .route("/register", post(login::register))
        .route("/auth", get(login::get_session))
        .route("/grab/local/:id/meals", get(gets::get_meals_from_local))
        .route("/grab/meal/:id/reviews", get(gets::get_reviews_for_meal))
        .route("/grab/meal/:id", get(gets::get_meal))
        .route("/grab/meals", get(gets::search_meals_by_tag))
        .route("/grab/review/:id", get(gets::get_feed_item))
        .route("/grab/user/:id", get(gets::get_user_item))
        .route("/grab/user/:id/followers", get(gets::get_user_followers))
        .route("/grab/user/:id/followed", get(gets::get_user_followed))
        .route("/grab/review/:id/comments", get(gets::get_comments_for_review))
        .route("/grab/feed/global", get(gets::get_global_feed))
        .route("/post/review", post(posts::add_review))
        .route("/post/comment", post(posts::add_comment))
        .route("/post/bio", post(posts::change_bio))
        .route("/post/local", post(posts::add_local))
        .route("/post/meal", post(posts::add_meal))
        .route("/post/follow", post(posts::follow))
        .route("/post/unfollow", post(posts::unfollow))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
