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
        let query_str: String = format!("SELECT * FROM requests WHERE request_id = $1");
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

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct RequestStatus {
    request_status_id: i32,
    request_status_name: String,
    request_status_description: String,
}