use std::env;
use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use async_trait::async_trait;

use crate::domain::constants::SALT_KEY;
use crate::domain::error::CommonError;
use crate::domain::models::token::Claim;
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
        let mut cloned_user = CreateUser {
            username: user.username,
            email: user.email,
            password: get_hashed_password(user.password).await?,
        };
        self.repository
            .create(&mut cloned_user)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
    async fn get_token(&self, mut login_user: LoginUser) -> Result<String, CommonError> {
        login_user.password = get_hashed_password(login_user.password).await?;
        let user = self
            .repository
            .get(&login_user)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        self.token_service.create(user.id).await
    }
    async fn validate_token(&self, token: String) -> Result<Claim, CommonError> {
        let claim = self.token_service.validate(token).await?;
        Ok(claim)
    }
}

async fn get_hashed_password(pass: String) -> Result<String, CommonError> {
    let salt_key = env::var(SALT_KEY).expect("SALT_KEY must be set");
    let salt = SaltString::encode_b64(salt_key.as_ref()).map_err(|_| CommonError {
        message: "salt key error".to_string(),
        code: 500,
    })?;

    let password_hash = Argon2::default()
        .hash_password(pass.as_ref(), &salt)
        .map_err(|_| CommonError {
            message: "salt key error".to_string(),
            code: 500,
        })?
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash).map_err(|_| CommonError {
        message: "salt key error".to_string(),
        code: 500,
    })?;
    match parsed_hash.hash {
        Some(hash) => Ok(hash.to_string()),
        None => Err(CommonError {
            message: "Hash not found".to_string(),
            code: 400,
        }),
    }
}
