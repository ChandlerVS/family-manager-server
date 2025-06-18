use std::future::Future;
use std::pin::Pin;

pub struct CreateUsersTableMigration {}

impl crate::migrations::Migration for CreateUsersTableMigration {
    fn name(&self) -> &str {
        return "m001_create_users_table";
    }

    fn up(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let create_table_sql = r#"
                CREATE TABLE IF NOT EXISTS users (
                    id SERIAL PRIMARY KEY,
                    first_name VARCHAR(255) NOT NULL,
                    last_name VARCHAR(255) NOT NULL,
                    email VARCHAR(255) NOT NULL UNIQUE,
                    password VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                )
            "#;
            sqlx::query(create_table_sql).execute(&pool).await?;

            let email_index_sql = "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)";
            sqlx::query(email_index_sql).execute(&pool).await?;

            let name_index_sql = "CREATE INDEX IF NOT EXISTS idx_users_name ON users(first_name, last_name)";
            sqlx::query(name_index_sql).execute(&pool).await?;

            Ok(())
        })
    }

    fn down(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let sql = r#"
                DROP TABLE IF EXISTS users;
            "#;

            sqlx::query(sql).execute(&pool).await?;

            Ok(())
        })
    }
}