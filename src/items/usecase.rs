use bson::oid::ObjectId;

use super::{
    entities::{InsertItemRequest, Item},
    repository,
};

pub async fn insert_one_item(req: InsertItemRequest) -> Result<ObjectId, String> {
    repository::insert_one_item(req).await
}

pub async fn find_one_item(id: ObjectId) -> Result<Item, String> {
    repository::find_one_item(id).await
}
