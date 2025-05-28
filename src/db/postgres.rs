use sqlx::{PgPool, FromRow};

pub struct PostgresClient {
    pool: PgPool
}

impl PostgresClient {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(PostgresClient { pool })
    }
    
    pub async fn get_item_by_id<T>(
        &self,
        resource_id: &i32,
        query_str: &str,
    ) -> Result<Option<T>, sqlx::Error> 
    where
        T: for <'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + std::marker::Sync,
    {
        let result = sqlx::query_as::<_, T>(
            &query_str
        )
        .bind(resource_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}