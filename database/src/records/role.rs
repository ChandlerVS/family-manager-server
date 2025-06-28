use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleRecord {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct RoleRecordMutation {
    pub name: String,
    pub description: Option<String>,
}

impl From<RoleRecord> for RoleRecordMutation {
    fn from(record: RoleRecord) -> Self {
        Self {
            name: record.name,
            description: record.description,
        }
    }
} 