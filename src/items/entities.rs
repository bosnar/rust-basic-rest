use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemBson {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32,
}

// ควรไปสร้างไว้ที่ model
#[derive(Debug, Serialize, Deserialize)]
pub struct InsertItemRequest {
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32,
}

// ส่วนของการ Implement
impl Item {
    pub fn new() -> Self {
        Item {
            _id: String::from(""),
            name: String::from(""),
            description: String::from(""),
            damage: 0,
            level_required: 0,
            price: 0,
        }
    }
}
