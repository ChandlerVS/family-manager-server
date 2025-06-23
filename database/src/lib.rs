use std::sync::{Arc, OnceLock};

use sqlx::PgPool;

pub mod error;
pub mod migrations;
pub mod records;
pub mod repositories;
pub struct DatabaseManager {
    pool: Arc<PgPool>,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, crate::error::DatabaseError> {
        let pool = PgPool::connect(database_url).await?;
        
        Ok(Self { pool: Arc::new(pool) })
    }

    pub async fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }

    pub async fn run_migrations(&self) -> Result<(), crate::error::DatabaseError> {
        let migration_manager = migrations::MigrationManager::new(self.get_pool().await);

        migration_manager.initialize().await?;
        migration_manager.run_migrations().await?;

        Ok(())
    }
}

static DATABASE_MANAGER: OnceLock<DatabaseManager> = OnceLock::new();

pub async fn initialize_database(database_url: &str) -> Result<(), crate::error::DatabaseError> {
    let manager = DatabaseManager::new(database_url).await?;
    DATABASE_MANAGER.set(manager).map_err(|_| crate::error::DatabaseError::ConnectionError(sqlx::Error::PoolClosed))?;
    Ok(())
}

pub fn get_database_manager() -> Option<&'static DatabaseManager> {
    DATABASE_MANAGER.get()
}
