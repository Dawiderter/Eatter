use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mysql_async::{prelude::*, Pool};
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
        .try_collect::<(u32, u32, String, u32)>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .collect::<Result<Vec<(u32, u32, String, u32)>, _>>()
        .map_err(|_| {
            info!("Schema error");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(|(id, price, name, local_id)| Meal {
            id,
            price,
            name,
            local_id,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "meals": res }))))
}
