use std::fmt::format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Transaction {
    transaction_id: i32,
    milestone_id: i32,
    amount: f32,
    token_name: String,
    deleted: bool,
    receiver_address: String,
    client_id: i32,
    freelancer_id: i32,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>
}

impl Transaction {
    pub async fn get_transaction(postgres: &PostgresClient, transaction_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM transaction WHERE transaction_id = $1");
        let transaction = postgres.get_item_by_id(
            &transaction_id,
            &query_str
        ).await.unwrap_or_else(|error| {
            eprintln!("Error getting transaction: {}", error);
            None
        });
        transaction
    }
}