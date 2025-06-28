use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleRecord {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleRecordMutation {
    pub user_id: i32,
    pub role_id: i32,
}

impl From<UserRoleRecord> for UserRoleRecordMutation {
    fn from(record: UserRoleRecord) -> Self {
        Self {
            user_id: record.user_id,
            role_id: record.role_id,
        }
    }
} 