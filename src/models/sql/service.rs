use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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

#[async_trait]
impl FetchableResource for Service {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, service_id: Self::IdType) -> Option<Service> {
        let query_str: String = format!("SELECT * FROM services WHERE service_id = {}", service_id);
        let service: Option<Service> = db.postgres.get_item_by_id(
            &service_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting service: {}", error);
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