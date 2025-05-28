use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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


#[async_trait]
impl FetchableResource for Review {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, review_id: Self::IdType) -> Option<Review> {
        let query_str: String = format!("SELECT * FROM sub_categories WHERE review_id = {}", review_id);
        let review: Option<Review> = db.postgres.get_item_by_id(
            &review_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting review: {}", error);
            None
        });
        review
    }
}