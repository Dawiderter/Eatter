use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mysql_async::{prelude::*, FromRowError, Pool};
use serde::Serialize;
use serde_json::json;
use tracing::info;

#[derive(Serialize, Debug)]
struct Meal {
    id: u32,
    price: u32,
    name: String,
    local_id: u32,
}

impl FromRow for Meal {
    fn from_row_opt(row: mysql_async::Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let err = || FromRowError(row.clone());

        let id: u32 = row.get_opt("id").ok_or_else(err)?.map_err(|_| err())?;
        let price: u32 = row.get_opt("price").ok_or_else(err)?.map_err(|_| err())?;
        let name: String = row.get_opt("name").ok_or_else(err)?.map_err(|_| err())?;
        let local_id: u32 = row
            .get_opt("local_id")
            .ok_or_else(err)?
            .map_err(|_| err())?;

        Ok(Self {
            id,
            price,
            name,
            local_id,
        })
    }
}

#[derive(Serialize, Debug)]
struct Review {
    id: u32,
    body: String,
    created_at: time::Date,
    score: u32,
    meal_id: u32,
    author_id: u32,
}

impl FromRow for Review {
    fn from_row_opt(row: mysql_async::Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let err = || FromRowError(row.clone());

        let id: u32 = row.get_opt("id").ok_or_else(err)?.map_err(|_| err())?;
        let body: String = row.get_opt("body").ok_or_else(err)?.map_err(|_| err())?;
        let created_at: time::Date = row
            .get_opt("created_at")
            .ok_or_else(err)?
            .map_err(|_| err())?;
        let score: u32 = row.get_opt("score").ok_or_else(err)?.map_err(|_| err())?;
        let meal_id: u32 = row.get_opt("meal_id").ok_or_else(err)?.map_err(|_| err())?;
        let author_id: u32 = row
            .get_opt("author_id")
            .ok_or_else(err)?
            .map_err(|_| err())?;

        Ok(Self {
            id,
            body,
            created_at,
            score,
            meal_id,
            author_id,
        })
    }
}

pub async fn get_meals_from_local(
    State(pool): State<Pool>,
    Path(body): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Meals from local: {:?}", body);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res: Vec<Meal> = conn
        .exec_iter(
            r"CALL getMealsFromLocal(:id)",
            params! {
                "id" => body,
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_collect::<Meal>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .collect::<Result<Vec<Meal>, _>>()
        .map_err(|e| {
            info!("Schema error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::OK, Json(json!({ "meals": res }))))
}

pub async fn get_reviews_from_meal(
    State(pool): State<Pool>,
    Path(body): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Reviews from meal: {:?}", body);

    let mut conn = pool
        .get_conn()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res: Vec<Review> = conn
        .exec_iter(
            r"CALL getReviewsForMeal(:id)",
            params! {
                "id" => body,
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_collect::<Review>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .collect::<Result<Vec<Review>, _>>()
        .map_err(|e| {
            info!("Schema error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .collect();

    Ok((StatusCode::OK, Json(json!({ "reviews": res }))))
}
