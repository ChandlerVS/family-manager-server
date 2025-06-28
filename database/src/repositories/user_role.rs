use crate::{get_database_manager, records::{user_role::{UserRoleRecord, UserRoleRecordMutation}, role::RoleRecord, PaginatedRecords}, repositories::Repository};
use std::sync::Arc;
use sqlx::PgPool;

pub struct UserRoleRepository {
    pool: Arc<PgPool>,
}

impl UserRoleRepository {
    pub async fn find_by_user_and_role(&self, user_id: i32, role_id: i32) -> Result<Option<UserRoleRecord>, crate::error::DatabaseError> {
        let user_role: Option<UserRoleRecord> = sqlx::query_as!(
            UserRoleRecord,
            r#"
            SELECT * FROM user_roles WHERE user_id = $1 AND role_id = $2
            "#, user_id, role_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user_role)
    }

    pub async fn find_roles_by_user_id(&self, user_id: i32) -> Result<Vec<RoleRecord>, crate::error::DatabaseError> {
        let roles: Vec<RoleRecord> = sqlx::query_as!(
            RoleRecord,
            r#"
            SELECT r.* FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            ORDER BY r.name
            "#, user_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(roles)
    }

    pub async fn find_users_by_role_id(&self, role_id: i32) -> Result<Vec<crate::records::user::UserRecord>, crate::error::DatabaseError> {
        let users: Vec<crate::records::user::UserRecord> = sqlx::query_as!(
            crate::records::user::UserRecord,
            r#"
            SELECT u.* FROM users u
            INNER JOIN user_roles ur ON u.id = ur.user_id
            WHERE ur.role_id = $1
            ORDER BY u.first_name, u.last_name
            "#, role_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(users)
    }

    pub async fn remove_role_from_user(&self, user_id: i32, role_id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2
            "#,
            user_id, role_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}

impl Repository<UserRoleRecord, UserRoleRecordMutation> for UserRoleRepository {
    fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    async fn get() -> Self {
        let pool = get_database_manager().unwrap().get_pool().await;

        Self::new(pool)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<UserRoleRecord>, crate::error::DatabaseError> {
        let user_role: Option<UserRoleRecord> = sqlx::query_as!(
            UserRoleRecord,
            r#"
            SELECT *
            FROM user_roles
            WHERE id = $1
            "#, id)
            .fetch_optional(&*self.pool)
            .await?;
    
        Ok(user_role)
    }

    async fn paginate(&self, page: u64, limit: u64) -> Result<PaginatedRecords<UserRoleRecord>, crate::error::DatabaseError> {
        let offset = (page - 1) * limit;
        
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM user_roles
            "#)
            .fetch_one(&*self.pool)
            .await?
            .unwrap_or(0);

        let records: Vec<UserRoleRecord> = sqlx::query_as!(
            UserRoleRecord,
            r#"
            SELECT *
            FROM user_roles
            ORDER BY user_id, role_id
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

    async fn create(&self, record: UserRoleRecordMutation) -> Result<UserRoleRecord, crate::error::DatabaseError> {
        let user_role: UserRoleRecord = sqlx::query_as!(
            UserRoleRecord,
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            record.user_id,
            record.role_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user_role)
    }

    async fn delete(&self, id: i32) -> Result<(), crate::error::DatabaseError> {
        sqlx::query!(
            r#"
            DELETE FROM user_roles WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: i32, record: UserRoleRecordMutation) -> Result<UserRoleRecord, crate::error::DatabaseError> {
        let user_role: UserRoleRecord = sqlx::query_as!(
            UserRoleRecord,
            r#"
            UPDATE user_roles SET user_id = $1, role_id = $2 WHERE id = $3 RETURNING *
            "#,
            record.user_id,
            record.role_id,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user_role)
    }
} 