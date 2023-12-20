// In utils/db.rs
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Error};
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct DatabasePool(pub Arc<DbPool>);

impl Clone for DatabasePool {
    fn clone(&self) -> Self {
        DatabasePool(Arc::clone(&self.0))
    }
}

impl TypeMapKey for DatabasePool {
    type Value = DatabasePool;
}

// Function to create the database connection pool
pub async fn create_pool(db_url: &str) -> Result<DbPool, Error> {
    let manager = PostgresConnectionManager::new_from_stringlike(db_url, NoTls)?;
    Pool::builder().build(manager).await
}
