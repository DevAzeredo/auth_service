use async_trait::async_trait;
use crate::domain::models::user::{CreateUser, User};
use crate::domain::error::CommonError;

#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError>;
    async fn get(&self, user_id: i32) -> Result<User, CommonError>;
}

