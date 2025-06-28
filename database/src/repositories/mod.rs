pub mod user;
pub mod permission;
pub mod role;
pub mod user_role;
pub mod role_permission;

use crate::records::PaginatedRecords;
use std::{future::Future, sync::Arc};
use sqlx::PgPool;

pub trait Repository<T, Mutation> {
    fn new(pool: Arc<PgPool>) -> Self;
    fn get() -> impl Future<Output = Self> + Send;

    fn find_by_id(&self, id: i32) -> impl Future<Output = Result<Option<T>, crate::error::DatabaseError>> + Send;
    fn paginate(&self, page: u64, limit: u64) -> impl Future<Output = Result<PaginatedRecords<T>, crate::error::DatabaseError>> + Send;
    fn create(&self, record: Mutation) -> impl Future<Output = Result<T, crate::error::DatabaseError>> + Send;
    fn update(&self, id: i32, record: Mutation) -> impl Future<Output = Result<T, crate::error::DatabaseError>> + Send;
    fn delete(&self, id: i32) -> impl Future<Output = Result<(), crate::error::DatabaseError>> + Send;
}
