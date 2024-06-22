use crate::domain::error::CommonError;
use crate::domain::models::token::Claim;
use crate::domain::services::token::TokenService;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

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
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        Ok(encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )?)
    }

    async fn validate(&self, token: String) -> Result<String, CommonError> {
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let token_data = decode::<Claim>(
            &*token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &Validation::default(),
        )?;
        Ok(format!(
            "sub: {}, exp: {}",
            token_data.claims.sub, token_data.claims.exp
        ))
    }
}
