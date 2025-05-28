use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;
use crate::models::sql::user::User;

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

impl Profile {
    pub async fn get_profile(postgres: &PostgresClient, profile_id: &i32) -> Option<Self> {
    let query_str: String = format!("SELECT * FROM profiles WHERE profile_id = $1");
    let profile: Option<Profile> = postgres.get_item_by_id(
        profile_id,
        &query_str
    ).await.unwrap_or_else(|e|{
        eprintln!("Database error: {:?}", e);
        None
    });
    profile
    }
}