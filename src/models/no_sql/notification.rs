use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;
use anyhow::Result;
use async_trait::async_trait;
use crate::db::mongo::MongoClient;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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

#[async_trait]
impl FetchableResource for Notification {
    type IdType = String;
    async fn fetch_by_id(db: &DbClients, notification_id: Self::IdType) -> Option<Notification> {
        let notification: Option<Notification> = db.mongo.get_by_field(
            "Notification",
            "_id",
            &notification_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting notification: {}", error);
            None
        });
        notification
    }
}
