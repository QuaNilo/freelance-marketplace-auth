use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

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

impl Order{
    pub async fn get_order(postgres: &PostgresClient, milestone_id: &i32) -> Option<Self>{
        let query_str: String = format!("SELECT * FROM orders WHERE order_id = $1");
        let order: Option<Order> = postgres.get_item_by_id(
            &milestone_id,
            &query_str
        ).await.unwrap_or_else(|error| {
            eprintln!("Error in database : {:?}", error);
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