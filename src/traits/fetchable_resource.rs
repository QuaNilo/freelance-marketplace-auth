use crate::db::postgres::PostgresClient;
use crate::db::mongo::MongoClient;

pub struct DbClients {
    pub postgres: PostgresClient,
    pub mongo: MongoClient,
}

#[async_trait::async_trait]
pub trait FetchableResource: Sized {
    type IdType;
    async fn fetch_by_id(db: &DbClients, id: Self::IdType) -> Option<Self>;
}