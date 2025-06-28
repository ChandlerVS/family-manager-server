use std::future::Future;
use std::pin::Pin;

pub struct CreatePermissionsTableMigration {}

impl crate::migrations::Migration for CreatePermissionsTableMigration {
    fn name(&self) -> &str {
        return "m002_create_permissions_table";
    }

    fn up(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let create_table_sql = r#"
                CREATE TABLE IF NOT EXISTS permissions (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    resource VARCHAR(255),
                    action VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                )
            "#;
            sqlx::query(create_table_sql).execute(&pool).await?;

            let name_index_sql = "CREATE INDEX IF NOT EXISTS idx_permissions_name ON permissions(name)";
            sqlx::query(name_index_sql).execute(&pool).await?;

            let resource_action_index_sql = "CREATE INDEX IF NOT EXISTS idx_permissions_resource_action ON permissions(resource, action)";
            sqlx::query(resource_action_index_sql).execute(&pool).await?;

            Ok(())
        })
    }

    fn down(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let sql = r#"
                DROP TABLE IF EXISTS permissions;
            "#;

            sqlx::query(sql).execute(&pool).await?;

            Ok(())
        })
    }
} 