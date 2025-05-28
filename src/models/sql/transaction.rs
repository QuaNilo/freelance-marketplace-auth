use std::fmt::format;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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

#[async_trait]
impl FetchableResource for Transaction {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, transaction_id: Self::IdType) -> Option<Transaction> {
        let query_str: String = format!("SELECT * FROM transactions WHERE transaction_id = {}", transaction_id);
        let transaction: Option<Transaction> = db.postgres.get_item_by_id(
            &transaction_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting transaction: {}", error);
            None
        });
        transaction
    }
}