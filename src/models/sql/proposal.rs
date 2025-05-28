use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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


#[async_trait]
impl FetchableResource for Proposal {
    type IdType = i32;
    async fn fetch_by_id(db: &DbClients, proposal_id: Self::IdType) -> Option<Proposal> {
        let query_str: String = format!("SELECT * FROM proposals WHERE proposal_id = {}", proposal_id);
        let proposal: Option<Proposal> = db.postgres.get_item_by_id(
            &proposal_id,
            &query_str,
        ).await.unwrap_or_else(|error|{
            eprintln!("Error getting proposal: {}", error);
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