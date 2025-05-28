use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;
use anyhow::Result;
use crate::db::mongo::MongoClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub user_id: i32,
    pub content: String,
    pub creation_date: DateTime,
    pub is_notified: bool
}

impl Notification {
    pub async fn get_notification(mongo: &MongoClient, resource_id : &String ) -> Result<Notification> {
        let notification = mongo.get_by_field("Notification", "_id", resource_id).await?;
        
        match notification {
            Some(notification) => {
                Ok(notification)
            }
            None => {
                Err(anyhow::Error::msg("Notification not found"))
            }
        }
    }
}
