use std::sync::Arc;

use async_trait::async_trait;
use bcrypt::{hash, DEFAULT_COST};

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, LoginUser, User};
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::token::TokenService;
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
    pub token_service: Arc<dyn TokenService>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>, token_service: Arc<dyn TokenService>) -> Self {
        UserServiceImpl {
            repository,
            token_service,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError> {
        let mut cloned = user.clone();
        cloned.password = hash(user.password, DEFAULT_COST)?;
        self.repository
            .create(&mut cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_token(&self, login_user: LoginUser) -> Result<String, CommonError> {
        let user = self
            .repository
            .get(&login_user)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        let token = self.token_service.create(user.id).await?;
        Ok(token)
    }
    async fn validate_token(&self, token: String) -> Result<String, CommonError> {
        let asd = self.token_service.validate(token).await?;
        Ok(asd)
    }
}
