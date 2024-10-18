use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};

use super::{
    entities::{InsertItemRequest, Item, ItemBson},
    repository,
};

pub async fn insert_one_item(req: InsertItemRequest) -> Result<ObjectId, String> {
    repository::insert_one_item(req).await
}

pub async fn find_item() -> Vec<Item> {
    repository::find_item().await
}

pub async fn find_one_item(id: ObjectId) -> Result<Item, String> {
    repository::find_one_item(id).await
}

pub async fn update_one_item(req: ItemBson) -> Result<UpdateResult, String> {
    repository::update_item(req).await
}

pub async fn delete_one_item(id: ObjectId) -> Result<DeleteResult, String> {
    repository::delete_item(id).await
}
