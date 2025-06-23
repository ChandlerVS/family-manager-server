use crate::{records::user::{UserRecordMutation, UserRecord}, repositories::Repository, records::PaginatedRecords};
use std::sync::Arc;
use sqlx::PgPool;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl Repository<UserRecord, UserRecordMutation> for UserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<UserRecord>, crate::error::DatabaseError> {
        let user: Option<UserRecord> = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#, id)
            .fetch_optional(&*self.pool)
            .await?;
    
        Ok(user)
    }

    async fn paginate(&self, page: u64, limit: u64) -> Result<PaginatedRecords<UserRecord>, crate::error::DatabaseError> {
        let offset = (page - 1) * limit;
        
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM users
            "#)
            .fetch_one(&*self.pool)
            .await?
            .unwrap_or(0);

        let records: Vec<UserRecord> = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT *
            FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(PaginatedRecords {
            records,
            total: total as u64,
            page,
            limit,
        })
    }

    async fn create(&self, record: UserRecordMutation) -> Result<UserRecord, crate::error::DatabaseError> {
        let user: UserRecord = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            record.first_name,
            record.last_name,
            record.email,
            record.password
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    async fn delete(&self, id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: i32, record: UserRecordMutation) -> Result<UserRecord, crate::error::DatabaseError> {
        let user: UserRecord = sqlx::query_as!(
            UserRecord,
            r#"
            UPDATE users SET first_name = $1, last_name = $2, email = $3, password = $4 WHERE id = $5 RETURNING *
            "#,
            record.first_name,
            record.last_name,
            record.email,
            record.password,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
