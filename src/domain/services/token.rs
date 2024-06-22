use crate::domain::error::CommonError;
use async_trait::async_trait;

#[async_trait]
pub trait TokenService: Sync + Send {
    async fn create(&self, user_id: i32) -> Result<String, CommonError>;
    async fn validate(&self, token: String) -> Result<String, CommonError>;
}
