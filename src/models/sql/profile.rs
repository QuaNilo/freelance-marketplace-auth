use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Profile {
    profile_id: i32,
    user_id: i32,
    first_name: String,
    last_name: String,
    bio: String,
    profile_picture_identifier: String,
    creation_date: DateTime<Utc>,
    edition_date: DateTime<Utc>,
}

#[async_trait]
impl FetchableResource for Profile {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, profile_id: Self::IdType) -> Option<Profile> {
        let query_str: String = format!("SELECT * FROM profiles WHERE profile_id = {}", profile_id);
        let profile: Option<Profile> = db.postgres.get_item_by_id(
            &profile_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting profile: {}", error);
            None
        });
        profile
    }
}