use crate::{get_database_manager, records::{role_permission::{RolePermissionRecord, RolePermissionRecordMutation}, permission::PermissionRecord, PaginatedRecords}, repositories::Repository};
use std::sync::Arc;
use sqlx::PgPool;

pub struct RolePermissionRepository {
    pool: Arc<PgPool>,
}

impl RolePermissionRepository {
    pub async fn find_by_role_and_permission(&self, role_id: i32, permission_id: i32) -> Result<Option<RolePermissionRecord>, crate::error::DatabaseError> {
        let role_permission: Option<RolePermissionRecord> = sqlx::query_as!(
            RolePermissionRecord,
            r#"
            SELECT * FROM role_permissions WHERE role_id = $1 AND permission_id = $2
            "#, role_id, permission_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(role_permission)
    }

    pub async fn find_permissions_by_role_id(&self, role_id: i32) -> Result<Vec<PermissionRecord>, crate::error::DatabaseError> {
        let permissions: Vec<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT p.* FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1
            ORDER BY p.name
            "#, role_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(permissions)
    }

    pub async fn find_roles_by_permission_id(&self, permission_id: i32) -> Result<Vec<crate::records::role::RoleRecord>, crate::error::DatabaseError> {
        let roles: Vec<crate::records::role::RoleRecord> = sqlx::query_as!(
            crate::records::role::RoleRecord,
            r#"
            SELECT r.* FROM roles r
            INNER JOIN role_permissions rp ON r.id = rp.role_id
            WHERE rp.permission_id = $1
            ORDER BY r.name
            "#, permission_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(roles)
    }

    pub async fn remove_permission_from_role(&self, role_id: i32, permission_id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM role_permissions WHERE role_id = $1 AND permission_id = $2
            "#,
            role_id, permission_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_user_permissions(&self, user_id: i32) -> Result<Vec<PermissionRecord>, crate::error::DatabaseError> {
        let permissions: Vec<PermissionRecord> = sqlx::query_as!(
            PermissionRecord,
            r#"
            SELECT DISTINCT p.* FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            INNER JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1
            ORDER BY p.name
            "#, user_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(permissions)
    }
}

impl Repository<RolePermissionRecord, RolePermissionRecordMutation> for RolePermissionRepository {
    fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    async fn get() -> Self {
        let pool = get_database_manager().unwrap().get_pool().await;

        Self::new(pool)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<RolePermissionRecord>, crate::error::DatabaseError> {
        let role_permission: Option<RolePermissionRecord> = sqlx::query_as!(
            RolePermissionRecord,
            r#"
            SELECT *
            FROM role_permissions
            WHERE id = $1
            "#, id)
            .fetch_optional(&*self.pool)
            .await?;
    
        Ok(role_permission)
    }

    async fn paginate(&self, page: u64, limit: u64) -> Result<PaginatedRecords<RolePermissionRecord>, crate::error::DatabaseError> {
        let offset = (page - 1) * limit;
        
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM role_permissions
            "#)
            .fetch_one(&*self.pool)
            .await?
            .unwrap_or(0);

        let records: Vec<RolePermissionRecord> = sqlx::query_as!(
            RolePermissionRecord,
            r#"
            SELECT *
            FROM role_permissions
            ORDER BY role_id, permission_id
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

    async fn create(&self, record: RolePermissionRecordMutation) -> Result<RolePermissionRecord, crate::error::DatabaseError> {
        let role_permission: RolePermissionRecord = sqlx::query_as!(
            RolePermissionRecord,
            r#"
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            record.role_id,
            record.permission_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(role_permission)
    }

    async fn delete(&self, id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM role_permissions WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: i32, record: RolePermissionRecordMutation) -> Result<RolePermissionRecord, crate::error::DatabaseError> {
        let role_permission: RolePermissionRecord = sqlx::query_as!(
            RolePermissionRecord,
            r#"
            UPDATE role_permissions SET role_id = $1, permission_id = $2 WHERE id = $3 RETURNING *
            "#,
            record.role_id,
            record.permission_id,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(role_permission)
    }
} 