use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;
use crate::models::sql::profile::Profile;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Request {
    request_id: i32,
    title: String,
    description: String,
    sub_category_id: i32,
    total_price: f32,
    tags: Vec<String>,
    deleted: bool,
    client_id: i32
}

impl Request {
    pub async fn get_request(postgres: &PostgresClient, request_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM profiles WHERE profile_id = $1");
        let request: Option<Request> = postgres.get_item_by_id(
            request_id,
            &query_str
        ).await.unwrap_or_else(|e|{
            eprintln!("Database error: {:?}", e);
            None
        });
        request
    }
}
