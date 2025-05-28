use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

impl User {
    pub async fn is_deleted(&self) -> bool {
        self.deleted
    }
}