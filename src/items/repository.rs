use super::entities::InsertItemRequest; // ใช้ super เพื่อบอก rust ว่าให้มองหา file นี้ใน parent directory
use crate::config::database::dbconnect;
// ใช้ crate เพื่อบอก rust ว่าให้มองหา file นี้ใน root directory
use bson::{doc, oid::ObjectId, Document};
use mongodb::{error::Error, results::InsertOneResult};
use tracing::log::info;

pub async fn insert_one_item(req: InsertItemRequest) -> Result<ObjectId, String> {
    let db = dbconnect().await.expect("error connection to database");

    let collection = db.collection::<Document>("items");

    let result: Result<InsertOneResult, Error> = collection
        .insert_one(doc! {
            "name": req.name,
            "description": req.description,
            "damage": req.damage,
            "level_required": req.level_required,
            "price": req.price,
        })
        .await;

    let inserted_id_bson = match result {
        Ok(inserted_id) => inserted_id.inserted_id,
        Err(e) => {
            info!("Error: {}", e);
            return Err(format!("Error: Insert one item failed"));
        }
    };

    // จัดการ Result ได้ทั้ง 2 แบบ
    // แบบที่ 1 ใช้ let if
    if let bson::Bson::ObjectId(id) = inserted_id_bson {
        info!("Inserted id: {}", id);
        return Ok(id);
    } else {
        return Err(format!("Error: Inserted id is not ObjectId"));
    }

    // แบบที่ 2 ใช้ match
    // match inserted_id_bson {
    //     bson::Bson::ObjectId(id) => {
    //         info!("Inserted id: {}", id);
    //         return Ok(id);
    //     }
    //     _ => {
    //         return Err(format!("Error: Inserted id is not ObjectId"));
    //     }
    // }
}
