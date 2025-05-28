use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;
use crate::models::sql::request::Request;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Service {
    service_id: i32,
    title: String,
    description: String,
    sub_category_id: i32,
    total_price: Option<f32>,
    tags: Vec<String>,
    deleted: bool,
    freelancer_id: i32,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
    service_status_id: i32,
}

impl Service {
    pub async fn get_service(postgres: &PostgresClient, service_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM services WHERE service_id = $1");
        let service: Option<Service> = postgres.get_item_by_id(
            service_id,
            &query_str
        ).await.unwrap_or_else(|e|{
            eprintln!("Database error: {:?}", e);
            None
        });
        service
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ServiceStatus {
    service_status_id: i32,
    service_status_name: String,
    service_status_description: String,
}