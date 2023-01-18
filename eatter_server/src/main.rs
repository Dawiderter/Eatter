use std::sync::Arc;

use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use clap::{Parser, Arg};
use eatter_server::{db::Database, login, posts, search, state::GlobalState};
use mysql_async::{Pool, OptsBuilder};
use tokio::sync::Mutex;
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::services::ServeDir;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
struct Args {
    #[arg(short,long)]
    pass: String,
}


#[tokio::main]
async fn main() {
    let db_pass = Args::parse().pass;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let key = Key::generate();
    let state = GlobalState {
        master_key: key.clone(),
        database: Database::new().await,
    };

    let pool = Pool::new(OptsBuilder::default().ip_or_hostname("localhost").user(Some("root")).pass(Some(db_pass)));
    let mut conn = pool.get_conn().await.unwrap();

    let app = Router::new()
        .route("/search", get(search::search))
        .route("/post", post(posts::input_post))
        .route("/login", post(login::login))
        .fallback_service(
            get_service(ServeDir::new("./eatter_frontend/dist")).handle_error(|err| async {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
            }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
