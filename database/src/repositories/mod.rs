pub mod user;
use crate::records::PaginatedRecords;

pub trait Repository<T, Mutation> {
    fn find_by_id(&self, id: i32) -> impl Future<Output = Result<Option<T>, crate::error::DatabaseError>> + Send;
    fn paginate(&self, page: u64, limit: u64) -> impl Future<Output = Result<PaginatedRecords<T>, crate::error::DatabaseError>> + Send;
    fn create(&self, record: Mutation) -> impl Future<Output = Result<T, crate::error::DatabaseError>> + Send;
    fn update(&self, id: i32, record: Mutation) -> impl Future<Output = Result<T, crate::error::DatabaseError>> + Send;
    fn delete(&self, id: i32) -> impl Future<Output = Result<(), crate::error::DatabaseError>> + Send;
}
