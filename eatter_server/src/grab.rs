use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{MySqlPool, query, query_as};
use tracing::{info, error, trace};

pub enum GrabError {
    DataBaseError(sqlx::Error),
}

#[derive(Serialize, Debug)]
struct Meal {
    id: u32,
    price: u32,
    name: String,
    local_id: u32,
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

#[derive(Serialize, Debug)]
struct FeedItem {
    r_id: i32,
    r_body: String,
    r_created_at: time::PrimitiveDateTime,
    r_score: i32,
    r_author_id: i32,
    m_id: i32,
    m_name: String,
    l_id: i32,
    l_name: String,
}


// pub async fn get_meals_from_local(
//     State(pool): State<Pool>,
//     Path(body): Path<u32>,
// ) -> Result<impl IntoResponse, StatusCode> {
//     info!("Meals from local: {:?}", body);

//     let mut conn = pool
//         .get_conn()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let res: Vec<Meal> = conn
//         .exec_iter(
//             r"CALL getMealsFromLocal(:id)",
//             params! {
//                 "id" => body,
//             },
//         )
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .try_collect::<Meal>()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .into_iter()
//         .collect::<Result<Vec<Meal>, _>>()
//         .map_err(|e| {
//             info!("Schema error: {:?}", e);
//             StatusCode::INTERNAL_SERVER_ERROR
//         })?;

//     Ok((StatusCode::OK, Json(json!({ "meals": res }))))
// }

// pub async fn get_reviews_from_meal(
//     State(pool): State<Pool>,
//     Path(body): Path<u32>,
// ) -> Result<impl IntoResponse, StatusCode> {
//     info!("Reviews from meal: {:?}", body);

//     let mut conn = pool
//         .get_conn()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let res: Vec<Review> = conn
//         .exec_iter(
//             r"CALL getReviewsForMeal(:id)",
//             params! {
//                 "id" => body,
//             },
//         )
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .try_collect::<Review>()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .into_iter()
//         .collect::<Result<Vec<Review>, _>>()
//         .map_err(|e| {
//             info!("Schema error: {:?}", e);
//             StatusCode::INTERNAL_SERVER_ERROR
//         })?
//         .into_iter()
//         .collect();

//     Ok((StatusCode::OK, Json(json!({ "reviews": res }))))
// }

// pub async fn get_review(
//     State(pool): State<Pool>,
//     Path(body): Path<u32>,
// ) -> Result<impl IntoResponse, StatusCode> {
//     info!("Reviews from meal: {:?}", body);

//     let mut conn = pool
//         .get_conn()
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let res: Review = conn
//         .exec_first(
//             r"CALL getPost(:id)",
//             params! {
//                 "id" => body,
//             },
//         )
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok((StatusCode::OK, Json(json!({ "review": res }))))
// }

pub async fn get_global_feed(State(pool): State<MySqlPool>) -> Result<impl IntoResponse, GrabError> {
    trace!("Global feed requested");

    let res = query_as!(FeedItem, "SELECT * FROM feed").fetch_all(&pool).await?;

    Ok(Json(json!(res)))
}

impl From<sqlx::Error> for GrabError {
    fn from(inner: sqlx::Error) -> Self {
        Self::DataBaseError(inner)
    }
}

impl IntoResponse for GrabError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataBaseError(inner) => {
                error!("Database error: {:?}", inner);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        .into_response()
    }
}