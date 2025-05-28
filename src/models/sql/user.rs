use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    user_id: i32,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
    deleted: bool,
    wallet_public_address: String,
    wallet_type_id: i32,
    last_login: DateTime<Utc>,
    role_id: i32,
    role_name: Option<String>,
    role_description: Option<String>,
}
#[async_trait]
impl FetchableResource for User {
    type IdType = i32;
    
    async fn fetch_by_id(db: &DbClients, user_id: Self::IdType) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM users WHERE user_id = $1");
        let user: Option<User> = db.postgres.get_item_by_id(
            &user_id,
            &query_str
        ).await.unwrap_or_else(|e|{
            eprintln!("Database error: {:?}", e);
            None
        });
        user
    }
}

impl User {
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}