pub mod user;
pub mod permission;
pub mod role;
pub mod user_role;
pub mod role_permission;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedRecords<T> {
    pub records: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
