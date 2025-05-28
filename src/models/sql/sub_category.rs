use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct SubCategory {
    sub_category_id: i32,
    sub_category_name: String,
    sub_category_description: String,
    category_id: i32,
    deleted: bool,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>
}

#[async_trait]
impl FetchableResource for SubCategory {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, sub_category_id: Self::IdType) -> Option<SubCategory> {
        let query_str: String = format!("SELECT * FROM sub_categories WHERE sub_category_id = {}", sub_category_id);
        let sub_category: Option<SubCategory> = db.postgres.get_item_by_id(
            &sub_category_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting sub category: {}", error);
            None
        });
        sub_category
    }
}