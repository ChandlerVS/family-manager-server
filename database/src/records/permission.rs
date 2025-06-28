use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRecord {
    pub id: i32,
    pub name: String,
    pub resource: Option<String>,
    pub action: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRecordMutation {
    pub name: String,
    pub resource: Option<String>,
    pub action: String,
}

impl From<PermissionRecord> for PermissionRecordMutation {
    fn from(record: PermissionRecord) -> Self {
        Self {
            name: record.name,
            resource: record.resource,
            action: record.action,
        }
    }
} 