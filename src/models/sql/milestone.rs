use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Milestone {
    milestone_id: i32,
    client_id: i32,
    freelancer_id: i32,
    milestone_tx_hash: String,
    milestone_text: String,
    reward_amount: f32,
    deleted: bool,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
    client_approved: bool,
    freelancer_approved: bool,
    milestone_status_id: i32,
}

impl Milestone {
    pub async fn get_milestone(postgres: &PostgresClient, milestone_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM milestones WHERE milestone_id = $1");
        let milestone: Option<Milestone> = postgres.get_item_by_id(
            milestone_id,
            &query_str
        ).await.unwrap_or_else(|e|{
            eprintln!("Database error: {:?}", e);
            None
        });
        milestone
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct MilestoneStatus {
    milestone_status_id: i32,
    milestone_status_name: String,
    milestone_status_description: String,
}