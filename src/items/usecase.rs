use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::{entities::InsertItemRequest, repository};

pub async fn insert_one_item(req: InsertItemRequest) -> impl IntoResponse {
    let result_object_id = repository::insert_one_item(req).await;
    match result_object_id {
        Ok(id) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": format!("Inserted id: {}", id),
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "failed",
                "message": e,
            })),
        )
            .into_response(),
    }
}
