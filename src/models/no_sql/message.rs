use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Message {
    sender_id: i32,
    receiver_id: i32,
    content: String,
    sent_time: DateTime<Utc>,
    received_time: Option<DateTime<Utc>>,
    is_delivered: bool,
    is_edited: bool,
    is_viewed: bool
}