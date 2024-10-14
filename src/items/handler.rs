use axum::{response::IntoResponse, Json};

use super::{entities::InsertItemRequest, usecase};

pub async fn insert_one_item(Json(req): Json<InsertItemRequest>) -> impl IntoResponse {
    usecase::insert_one_item(req).await
}
