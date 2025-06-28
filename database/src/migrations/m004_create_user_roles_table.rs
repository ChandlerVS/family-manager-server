use std::future::Future;
use std::pin::Pin;

pub struct CreateUserRolesTableMigration {}

impl crate::migrations::Migration for CreateUserRolesTableMigration {
    fn name(&self) -> &str {
        return "m004_create_user_roles_table";
    }

    fn up(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let create_user_roles_table_sql = r#"
                CREATE TABLE IF NOT EXISTS user_roles (
                    id SERIAL PRIMARY KEY,
                    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    UNIQUE(user_id, role_id)
                )
            "#;
            sqlx::query(create_user_roles_table_sql).execute(&pool).await?;

            let user_roles_user_index_sql = "CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id)";
            sqlx::query(user_roles_user_index_sql).execute(&pool).await?;

            let user_roles_role_index_sql = "CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles(role_id)";
            sqlx::query(user_roles_role_index_sql).execute(&pool).await?;

            let user_roles_composite_index_sql = "CREATE INDEX IF NOT EXISTS idx_user_roles_user_role ON user_roles(user_id, role_id)";
            sqlx::query(user_roles_composite_index_sql).execute(&pool).await?;

            Ok(())
        })
    }

    fn down(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let drop_user_roles_sql = "DROP TABLE IF EXISTS user_roles";
            sqlx::query(drop_user_roles_sql).execute(&pool).await?;

            Ok(())
        })
    }
}