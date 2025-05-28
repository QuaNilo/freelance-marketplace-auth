use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;
use crate::models::sql::category::Category;

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

impl SubCategory {
    pub async fn get_sub_category(postgres: &PostgresClient, sub_category_id: i32) -> Option<SubCategory> {
        let query_str: String = format!("SELECT * FROM sub_categories WHERE sub_category_id = {}", sub_category_id);
        let sub_category: Option<SubCategory> = postgres.get_item_by_id(
            &sub_category_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting sub category: {}", error);
            None
        });
        sub_category
    }
}