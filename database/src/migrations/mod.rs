use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use sqlx::{PgPool, Row};
use tracing::info;

mod m001_create_users_table;
mod m002_create_permissions_table;
mod m003_create_roles_table;
mod m004_create_user_roles_table;

pub struct MigrationManager {
    pool: Arc<PgPool>,
}

impl MigrationManager {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn initialize(&self) -> Result<(), crate::error::DatabaseError> {
        info!("Initializing migrations");
        let create_table_sql = r#"
            CREATE TABLE IF NOT EXISTS migrations (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                applied_at TIMESTAMP WITH TIME ZONE
            );
        "#;

        sqlx::query(create_table_sql).execute(&*self.pool).await?;
        Ok(())
    }

    fn get_available_migrations(&self) -> Vec<Box<dyn Migration>> {
        vec![
            Box::new(m001_create_users_table::CreateUsersTableMigration {}),
            Box::new(m002_create_permissions_table::CreatePermissionsTableMigration {}),
            Box::new(m003_create_roles_table::CreateRolesTableMigration {}),
            Box::new(m004_create_user_roles_table::CreateUserRolesTableMigration {}),
        ]
    }

    async fn get_applied_migrations(&self) -> Result<Vec<String>, crate::error::DatabaseError> {
        let rows = sqlx::query(
            "SELECT name FROM migrations WHERE applied_at IS NOT NULL ORDER BY id"
        )
        .fetch_all(&*self.pool)
        .await?;

        let mut applied = Vec::new();
        for row in rows {
            applied.push(row.try_get("name")?);
        }
        Ok(applied)
    }

    pub async fn run_migrations(&self) -> Result<(), crate::error::DatabaseError> {
        info!("Running migrations");
        
        let available_migrations = self.get_available_migrations();
        let applied_migrations = self.get_applied_migrations().await?;

        for migration in available_migrations {
            let migration_name = migration.name();
            
            if !applied_migrations.contains(&migration_name.to_string()) {
                info!("Running migration: {}", migration_name);
                
                migration.up(&*self.pool).await?;
                
                sqlx::query(
                    "INSERT INTO migrations (name, applied_at) VALUES ($1, NOW())"
                )
                .bind(migration_name)
                .execute(&*self.pool)
                .await?;
                
                info!("Successfully applied migration: {}", migration_name);
            } else {
                info!("Migration {} already applied, skipping", migration_name);
            }
        }
        
        info!("All migrations completed");
        Ok(())
    }

    pub async fn rollback_last(&self) -> Result<(), crate::error::DatabaseError> {
        info!("Rolling back last migration");
        
        let last_migration = sqlx::query(
            "SELECT name FROM migrations WHERE applied_at IS NOT NULL ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&*self.pool)
        .await?;

        match last_migration {
            Some(row) => {
                let migration_name: String = row.try_get("name")?;
                
                info!("Rolling back migration: {}", migration_name);
                
                let available_migrations = self.get_available_migrations();
                for migration in available_migrations {
                    if migration.name() == migration_name {
                        migration.down(&*self.pool).await?;
                        break;
                    }
                }
                
                sqlx::query(
                    "DELETE FROM migrations WHERE name = $1"
                )
                .bind(&migration_name)
                .execute(&*self.pool)
                .await?;
                
                info!("Successfully rolled back migration: {}", migration_name);
                Ok(())
            }
            None => {
                info!("No migrations to rollback");
                Ok(())
            }
        }
    }

    pub async fn get_migration_status(&self) -> Result<Vec<MigrationStatus>, crate::error::DatabaseError> {
        let available_migrations = self.get_available_migrations();
        let applied_migrations = self.get_applied_migrations().await?;

        let mut status = Vec::new();
        for migration in available_migrations {
            let migration_name = migration.name();
            let is_applied = applied_migrations.contains(&migration_name.to_string());
            
            status.push(MigrationStatus {
                name: migration_name.to_string(),
                applied: is_applied,
            });
        }
        
        Ok(status)
    }
}

#[derive(Debug)]
pub struct MigrationStatus {
    pub name: String,
    pub applied: bool,
}

pub trait Migration {
    fn name(&self) -> &str;
    fn up(&self, pool: &PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>>;
    fn down(&self, pool: &PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>>;
}