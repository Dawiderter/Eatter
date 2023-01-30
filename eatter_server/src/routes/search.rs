use axum::{extract::{State, Query}, response::IntoResponse, Json, http::StatusCode, Router, body::Body, routing::get};
use serde::Deserialize;
use serde_json::json;
use sqlx::{MySqlPool, query_as, query};
use tracing::trace;

use crate::{routes::meal::MealItem, state::GlobalState};
use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    tags: String,
    sort: Option<String>,
}

pub fn search_router() -> Router<GlobalState, Body> {
    Router::new()
        .route("/", get(search))
}

async fn search(
    State(pool): State<MySqlPool>,
    Query(search_query): Query<SearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Search {:?}", search_query);

    let tags = split_tags(&search_query.tags);
    let sort = map_sort(search_query.sort.as_deref());
    let query = make_query(tags.len(), sort);

    let mut query = query_as(&query);

    for tag in tags {
        query = query.bind(tag.to_lowercase());
    }

    let res : Vec<MealItem> = query.fetch_all(&pool).await?;

    Ok(Json(json!(res)))
}

fn make_query(n_tags: usize, sort: &str) -> String {
    let mut query = "SELECT m.* FROM meal_items m JOIN meals_tags mt ON m.m_id = mt.meal_id JOIN tags t ON t.id = mt.tag_id WHERE ".to_owned();

    for _ in 0..n_tags {
        query += "t.name LIKE ? AND ";
    }

    query += "1 ";

    query += "ORDER BY ";
    query += sort;

    query
}

fn map_sort(sort: Option<&str>) -> &'static str {
    match sort {
        Some("name") => "m_name",
        Some("price") => "m_price",
        _ => "m_name"
    }
}

fn split_tags(tags: &str) -> Vec<&str> {
    tags.split(',').collect()
}