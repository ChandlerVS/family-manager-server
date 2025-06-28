use crate::{get_database_manager, records::{role::{RoleRecord, RoleRecordMutation}, PaginatedRecords}, repositories::Repository};
use std::sync::Arc;
use sqlx::PgPool;

pub struct RoleRepository {
    pool: Arc<PgPool>,
}

impl RoleRepository {
    pub async fn find_by_name(&self, name: &str) -> Result<Option<RoleRecord>, crate::error::DatabaseError> {
        let role: Option<RoleRecord> = sqlx::query_as!(
            RoleRecord,
            r#"
            SELECT * FROM roles WHERE name = $1
            "#, name)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(role)
    }
}

impl Repository<RoleRecord, RoleRecordMutation> for RoleRepository {
    fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    async fn get() -> Self {
        let pool = get_database_manager().unwrap().get_pool().await;

        Self::new(pool)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<RoleRecord>, crate::error::DatabaseError> {
        let role: Option<RoleRecord> = sqlx::query_as!(
            RoleRecord,
            r#"
            SELECT *
            FROM roles
            WHERE id = $1
            "#, id)
            .fetch_optional(&*self.pool)
            .await?;
    
        Ok(role)
    }

    async fn paginate(&self, page: u64, limit: u64) -> Result<PaginatedRecords<RoleRecord>, crate::error::DatabaseError> {
        let offset = (page - 1) * limit;
        
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM roles
            "#)
            .fetch_one(&*self.pool)
            .await?
            .unwrap_or(0);

        let records: Vec<RoleRecord> = sqlx::query_as!(
            RoleRecord,
            r#"
            SELECT *
            FROM roles
            ORDER BY name
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

    async fn create(&self, record: RoleRecordMutation) -> Result<RoleRecord, crate::error::DatabaseError> {
        let role: RoleRecord = sqlx::query_as!(
            RoleRecord,
            r#"
            INSERT INTO roles (name, description)
            VALUES ($1, $2)
            RETURNING *
            "#,
            record.name,
            record.description
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(role)
    }

    async fn delete(&self, id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM roles WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: i32, record: RoleRecordMutation) -> Result<RoleRecord, crate::error::DatabaseError> {
        let role: RoleRecord = sqlx::query_as!(
            RoleRecord,
            r#"
            UPDATE roles SET name = $1, description = $2 WHERE id = $3 RETURNING *
            "#,
            record.name,
            record.description,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(role)
    }
} 