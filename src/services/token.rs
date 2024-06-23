use std::env;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::domain::constants::SECRET_KEY_TOKEN;
use crate::domain::error::CommonError;
use crate::domain::models::token::Claim;
use crate::domain::services::token::TokenService;

#[derive(Clone)]
pub struct TokenServiceImpl {}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn create(&self, user_id: i32) -> Result<String, CommonError> {
        let expiration = Utc::now() + Duration::seconds(3600);
        let claim = Claim {
            sub: user_id.to_string(),
            exp: expiration.timestamp(),
        };
        let secret_key = env::var(SECRET_KEY_TOKEN).expect("SECRET_KEY must be set");
        Ok(encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )?)
    }
    async fn validate(&self, token: String) -> Result<Claim, CommonError> {
        let secret_key = env::var(SECRET_KEY_TOKEN).expect("SECRET_KEY must be set");
        let token_data = decode::<Claim>(
            &*token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &Validation::default(),
        )?;

        let now = Utc::now().timestamp();
        if token_data.claims.exp < now {
            return Err(CommonError {
                message: "Token has expired".to_string(),
                code: 401,
            });
        }
        Ok(token_data.claims)
    }
}
