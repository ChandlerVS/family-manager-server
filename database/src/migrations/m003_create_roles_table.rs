use std::future::Future;
use std::pin::Pin;

pub struct CreateRolesTableMigration {}

impl crate::migrations::Migration for CreateRolesTableMigration {
    fn name(&self) -> &str {
        return "m003_create_roles_table";
    }

    fn up(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let create_roles_table_sql = r#"
                CREATE TABLE IF NOT EXISTS roles (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL UNIQUE,
                    description TEXT,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                )
            "#;
            sqlx::query(create_roles_table_sql).execute(&pool).await?;

            let create_role_permissions_table_sql = r#"
                CREATE TABLE IF NOT EXISTS role_permissions (
                    id SERIAL PRIMARY KEY,
                    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                    permission_id INTEGER NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    UNIQUE(role_id, permission_id)
                )
            "#;
            sqlx::query(create_role_permissions_table_sql).execute(&pool).await?;

            let roles_name_index_sql = "CREATE INDEX IF NOT EXISTS idx_roles_name ON roles(name)";
            sqlx::query(roles_name_index_sql).execute(&pool).await?;

            let role_permissions_role_index_sql = "CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions(role_id)";
            sqlx::query(role_permissions_role_index_sql).execute(&pool).await?;

            let role_permissions_permission_index_sql = "CREATE INDEX IF NOT EXISTS idx_role_permissions_permission_id ON role_permissions(permission_id)";
            sqlx::query(role_permissions_permission_index_sql).execute(&pool).await?;

            Ok(())
        })
    }

    fn down(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let drop_role_permissions_sql = "DROP TABLE IF EXISTS role_permissions";
            sqlx::query(drop_role_permissions_sql).execute(&pool).await?;

            let drop_roles_sql = "DROP TABLE IF EXISTS roles";
            sqlx::query(drop_roles_sql).execute(&pool).await?;

            Ok(())
        })
    }
} 