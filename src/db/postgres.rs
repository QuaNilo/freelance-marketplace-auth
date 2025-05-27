use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};

pub struct PostgresClient {
    pool: PgPool
}
#[derive(Debug, FromRow)]
pub struct User {
    user_id: i32,
    creation_date: DateTime<Utc>,
    edition_date: Option<DateTime<Utc>>,
    deleted: bool,
    wallet_public_address: String,
    wallet_type_id: i32,
    last_login: DateTime<Utc>,
    role_id: i32,
}

impl PostgresClient {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(PostgresClient { pool })
    }
    
    pub async fn get_item_by_id<T>(
        &self, 
        resource_id: i32,
        table_name: &str,
        id_column: &str
    ) -> Result<Option<T>, sqlx::Error> 
    where
        T: for <'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + std::marker::Sync,
    {
        let query_str = format!("SELECT * FROM {} WHERE {} = $1", table_name, id_column);
        let result = sqlx::query_as::<_, T>(
            &query_str
        )
        .bind(resource_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}