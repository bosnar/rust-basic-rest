use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use serde_json::json;

use super::{
    entities::{InsertItemRequest, ItemBson},
    usecase,
};

pub async fn insert_one_item(Json(req): Json<InsertItemRequest>) -> impl IntoResponse {
    let result_object_id = usecase::insert_one_item(req).await;
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

pub async fn find_items() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": usecase::find_item().await,
        })),
    )
        .into_response()
}

// note ... Path(id): Path<ObjectId>
// generic struct
pub async fn find_one_item(Path(id): Path<ObjectId>) -> impl IntoResponse {
    match usecase::find_one_item(id).await {
        Ok(item) => {
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "data": item,
                })),
            )
        }
        .into_response(),
        Err(e) => {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "failed",
                    "message": e,
                })),
            )
        }
        .into_response(),
    }
}

pub async fn update_item(
    Path(id): Path<ObjectId>,
    Json(req): Json<InsertItemRequest>,
) -> impl IntoResponse {
    // สร้าง item ใหม่จากข้อมูลที่รับเข้ามา
    let item_to_update = ItemBson {
        _id: id,
        name: req.name,
        description: req.description,
        damage: req.damage,
        level_required: req.level_required,
        price: req.price,
    };

    match usecase::update_one_item(item_to_update).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Updated item",
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

pub async fn delete_item(Path(id): Path<ObjectId>) -> impl IntoResponse {
    match usecase::delete_one_item(id).await {
        Ok(r) => (StatusCode::OK, Json(r)).into_response(),
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
