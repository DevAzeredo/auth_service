use async_trait::async_trait;
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::models::user::{User, CreateUser};


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User>;
    async fn get(&self, user_id: i32) -> RepositoryResult<User>;
}
