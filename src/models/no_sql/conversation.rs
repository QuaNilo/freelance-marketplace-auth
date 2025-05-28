use serde::{Deserialize, Serialize};
use crate::db::mongo::MongoClient;
use crate::models::no_sql::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    participants: Vec<i32>,
    messages: Vec<Message>
}

impl Conversation {
    pub async fn get_conversation(mongo: &MongoClient, conversation_id: &str) -> Option<Conversation> {
        let conversation: Option<Conversation> = mongo.get_by_field(
            "Conversation",
            "_id",
            conversation_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting conversation: {}", error);
            None
        });
        conversation
    }
}