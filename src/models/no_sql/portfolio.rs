use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::db::mongo::MongoClient;

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

impl Portfolio {
    pub async fn get_portfolio(mongo: &MongoClient, portfolio_id: &str) -> Option<Portfolio> {
        let portfolio: Option<Portfolio> = mongo.get_by_field(
            "Portfolio",
            "_id",
            portfolio_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting portfolio: {}", error);
            None
        });
        portfolio
    }
}