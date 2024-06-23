use actix_web::{web, Result};

use crate::api::dto::user::{CreateUserDTO, LoginUserDTO, TokenDTO};
use crate::domain::error::ApiError;
use crate::domain::models::token::Claim;
use crate::domain::models::user::CreateUser;
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<String>, ApiError> {
    let create_user: CreateUser = post_data.into_inner().into();
    let password = create_user.clone().password;
    let username = create_user.clone().username;
    user_service.create(create_user).await?;
    let token = user_service
        .get_token(LoginUserDTO { username, password }.into())
        .await?;
    Ok(web::Json(token))
}

pub async fn login_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<LoginUserDTO>,
) -> Result<web::Json<String>, ApiError> {
    let token = user_service
        .get_token(post_data.into_inner().into())
        .await?;
    Ok(web::Json(token))
}

pub async fn validate_token_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<TokenDTO>,
) -> Result<web::Json<Claim>, ApiError> {
    let token = user_service.validate_token(post_data.token.clone()).await?;
    Ok(web::Json(token))
}
