use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use clap::Parser;
use eatter_server::{login, posts, search};
use mysql_async::{OptsBuilder, Pool};

use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    pass: String,
}

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub database: Pool,
}

#[tokio::main]
async fn main() {
    let db_pass = Args::parse().pass;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "eatter_server=debug".to_owned()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = Pool::new(
        OptsBuilder::default()
            .ip_or_hostname("localhost")
            .user(Some("root"))
            .pass(Some(db_pass))
            .db_name(Some("eatter")),
    );

    if pool.clone().get_conn().await.is_ok() {
        info!("Connection with database established");
    } else {
        info!("Database connection error");
    }

    let state = GlobalState { database: pool };

    let app = Router::new()
        .route("/search", get(search::search))
        .route("/login", post(login::create_session))
        .route("/auth/:tok", get(login::get_session))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
