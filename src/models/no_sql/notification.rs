use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;
use anyhow::Result;
use crate::config::Settings;
use crate::db::mongo::MongoClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub notification_id: i32,
    pub user_id: i32,
    pub content: String,
    pub creation_date: DateTime,
    pub is_notified: bool
}

impl Notification {
    pub fn new(notification_id: i32, user_id: i32, content: String, creation_date: DateTime, is_notified: bool) -> Self {
        Notification {
            notification_id,
            user_id,
            content,
            creation_date,
            is_notified,
        }
    }
    
    pub async fn get_notification(resource_id : &String ) -> Result<Notification> {
        let settings = Settings::new();
        let mongo: MongoClient = MongoClient::new(
            &settings.mongo.connection_string,
            &settings.mongo.database_name
        ).await?;
        
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
