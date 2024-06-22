use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, LoginUser, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError>;
    async fn get_token(&self, login_user: LoginUser) -> Result<String, CommonError>;
    async fn validate_token(&self, token: String) -> Result<String, CommonError>;
}
