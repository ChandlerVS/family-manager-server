use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionRecord {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionRecordMutation {
    pub role_id: i32,
    pub permission_id: i32,
}

impl From<RolePermissionRecord> for RolePermissionRecordMutation {
    fn from(record: RolePermissionRecord) -> Self {
        Self {
            role_id: record.role_id,
            permission_id: record.permission_id,
        }
    }
} 