use crate::domain::models::user::{CreateUser, LoginUser, User};
use crate::domain::repositories::repository::RepositoryResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User>;
    async fn get(&self, login_user: &LoginUser) -> RepositoryResult<User>;
}
