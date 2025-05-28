use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Order {
    order_id: i32,
    service_id: i32,
    client_id: i32,
    deleted: bool,
    order_status_id: i32,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
}

#[async_trait]
impl FetchableResource for Order {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, order_id: Self::IdType) -> Option<Order> {
        let query_str: String = format!("SELECT * FROM orders WHERE order_id = {}", order_id);
        let order: Option<Order> = db.postgres.get_item_by_id(
            &order_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting order: {}", error);
            None
        });
        order
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct OrderStatus {
    order_status_id: i32,
    order_status_name: String,
    order_status_description: String,
}