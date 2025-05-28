use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::postgres::PostgresClient;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Proposal {
    proposal_id: i32,
    request_id: i32,
    freelancer_id: i32,
    proposal_status_id: i32,
    deleted: bool,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
}

impl Proposal {
    pub async fn get_proposal(postgres: &PostgresClient, proposal_id: &i32) -> Option<Self> {
        let query_str: String = format!("SELECT * FROM proposals WHERE proposal_id = $1");
        let proposal: Option<Proposal> = postgres.get_item_by_id(
            &proposal_id,
            &query_str
        ).await.unwrap_or_else(|e| {
            eprintln!("Error fetching proposal: {:?}", e);
            None
        });
        proposal
    }
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProposalStatus {
    proposal_status_id: i32,
    proposal_status_name: String,
    proposal_status_description: String,
}