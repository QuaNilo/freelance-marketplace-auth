use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Review {
    review_id: i32,
    reviewee_id: i32,
    reviewer_id: i32,
    rating: f32,
    comment: String,
    delete: bool,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
}

impl Review {
    pub async fn get_review(postgres: &PostgresClient, review_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM reviews WHERE review_id = {}", review_id);
        let review: Option<Self> = postgres.get_item_by_id(
            &review_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting review: {}", error);
            None
        });
        review
    }
}