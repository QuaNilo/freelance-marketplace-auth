use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    file_storage_identifier: String,
    file_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    images: Vec<File>,
    attachments: Vec<File>,
    project_title: String,
    description: String,
    start_date: DateTime<Utc>,
    completion_date: DateTime<Utc>,
    tech_stack: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    user_id: i32,
    projects: Vec<Project>
}

#[async_trait]
impl FetchableResource for Portfolio {
    type IdType = String;
    async fn fetch_by_id(db: &DbClients, portfolio_id: Self::IdType) -> Option<Portfolio> {
        let portfolio: Option<Portfolio> = db.mongo.get_by_field(
            "Portfolio",
            "_id",
            &portfolio_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting portfolio: {}", error);
            None
        });
        portfolio
    }
}