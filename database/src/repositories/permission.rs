use crate::{get_database_manager, records::{permission::{PermissionRecord, PermissionRecordMutation}, PaginatedRecords}, repositories::Repository};
use std::sync::Arc;
use sqlx::PgPool;

pub struct PermissionRepository {
    pool: Arc<PgPool>,
}

impl PermissionRepository {
    pub async fn find_by_name(&self, name: &str) -> Result<Option<PermissionRecord>, crate::error::DatabaseError> {
        let permission: Option<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT * FROM permissions WHERE name = $1
            "#, name)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(permission)
    }

    pub async fn find_by_resource_and_action(&self, resource: &str, action: &str) -> Result<Option<PermissionRecord>, crate::error::DatabaseError> {
        let permission: Option<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT * FROM permissions WHERE resource = $1 AND action = $2
            "#, resource, action)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(permission)
    }

    pub async fn find_by_resource(&self, resource: &str) -> Result<Vec<PermissionRecord>, crate::error::DatabaseError> {
        let permissions: Vec<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT * FROM permissions WHERE resource = $1 ORDER BY name
            "#, resource)
            .fetch_all(&*self.pool)
            .await?;

        Ok(permissions)
    }
}

impl Repository<PermissionRecord, PermissionRecordMutation> for PermissionRepository {
    fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    async fn get() -> Self {
        let pool = get_database_manager().unwrap().get_pool().await;

        Self::new(pool)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PermissionRecord>, crate::error::DatabaseError> {
        let permission: Option<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT *
            FROM permissions
            WHERE id = $1
            "#, id)
            .fetch_optional(&*self.pool)
            .await?;
    
        Ok(permission)
    }

    async fn paginate(&self, page: u64, limit: u64) -> Result<PaginatedRecords<PermissionRecord>, crate::error::DatabaseError> {
        let offset = (page - 1) * limit;
        
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM permissions
            "#)
            .fetch_one(&*self.pool)
            .await?
            .unwrap_or(0);

        let records: Vec<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT *
            FROM permissions
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

    async fn create(&self, record: PermissionRecordMutation) -> Result<PermissionRecord, crate::error::DatabaseError> {
        let permission: PermissionRecord = sqlx::query_as!(
            PermissionRecord,
            r#"
            INSERT INTO permissions (name, resource, action)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            record.name,
            record.resource,
            record.action
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(permission)
    }

    async fn delete(&self, id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM permissions WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: i32, record: PermissionRecordMutation) -> Result<PermissionRecord, crate::error::DatabaseError> {
        let permission: PermissionRecord = sqlx::query_as!(
            PermissionRecord,
            r#"
            UPDATE permissions SET name = $1, resource = $2, action = $3 WHERE id = $4 RETURNING *
            "#,
            record.name,
            record.resource,
            record.action,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(permission)
    }
} 