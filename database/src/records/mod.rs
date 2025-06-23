pub mod user;

pub struct PaginatedRecords<T> {
    pub records: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
