use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::models::no_sql::message::Message;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    participants: Vec<i32>,
    messages: Vec<Message>
}
#[async_trait]
impl FetchableResource for Conversation {
    type IdType = String;
    async fn fetch_by_id(db: &DbClients, conversation_id: Self::IdType) -> Option<Conversation> {
        let conversation: Option<Conversation> = db.mongo.get_by_field(
            "Conversation",
            "_id",
            &conversation_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting conversation: {}", error);
            None
        });
        conversation
    }
}