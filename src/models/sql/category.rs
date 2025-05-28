use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    category_id: i32,
    category_name: Option<String>,
    category_description: Option<String>,
    deleted: bool,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
}

impl Category {
    pub async fn get_category(postgres: &PostgresClient, category_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM categories WHERE category_id = {}", category_id);
        let category: Option<Category> = postgres.get_item_by_id(
            &category_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting category: {}", error);
            None
        });
        category
    }
}