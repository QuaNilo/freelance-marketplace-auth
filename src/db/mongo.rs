use mongodb::bson::DateTime;
use mongodb::{Client, Database, options::ClientOptions, Collection};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use anyhow::Result;
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,

    pub sent_time: DateTime,
    pub received_time: DateTime,

    pub is_edited: bool,
    pub is_viewed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub conversation_id: i32,
    pub messages: Vec<Message>,
}

pub struct MongoClient {
    db: Database,
}

impl MongoClient {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        let db = client.database(&db_name);
        Ok(MongoClient { db })
    }
    
    pub async fn get_by_field<T>(
        &self,
        collection: &str,
        field: &str,
        value: &str,
    ) -> Result<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de> + Unpin + Send + Sync,
    {
        let col: Collection<T> = self.db.collection(collection);
        let filter = if field == "_id" {
        let obj_id = ObjectId::parse_str(value)?;
            doc! { "_id": obj_id }
        } else {
            doc! { field: value }
        };
        let result = col.find_one(filter).await?;
        Ok(result)
    }
}
