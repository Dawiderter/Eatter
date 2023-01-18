use crate::{db::Database, posts::Post};
use askama::Template;
use axum::{extract::State, response::IntoResponse};
use tower_cookies::{Cookies, Key};

pub async fn base(
    cookies: Cookies,
    State(key): State<Key>,
    State(database): State<Database>,
) -> impl IntoResponse {
    let cookie = cookies.signed(&key).get("login");
    match cookie {
        Some(cookie) => BaseFeedTemplate {
            posts: database.posts().await.collect(),
            logged: true,
        },
        None => BaseFeedTemplate {
            posts: database.posts().await.collect(),
            logged: false,
        },
    }
}

#[derive(Template)]
#[template(path = "base_feed.html")]
pub struct BaseFeedTemplate {
    pub posts: Vec<Post>,
    pub logged: bool,
}
