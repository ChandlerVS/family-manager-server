use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecordMutation {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl From<UserRecord> for UserRecordMutation {
    fn from(record: UserRecord) -> Self {
        Self {
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            password: record.password,
        }
    }
}
