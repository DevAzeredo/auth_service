use crate::api::dto::user::{CreateUserDTO, LoginUserDTO};
use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;
use actix_web::{web, Result};

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<String>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    let login_token = user_service
        .get_token(
            LoginUserDTO {
                username: user.username,
                password: user.password,
            }
            .into(),
        )
        .await?;
    Ok(web::Json(login_token))
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
    post_data: web::Data<String>,
) -> Result<web::Json<String>, ApiError> {
    let token = user_service.validate_token(post_data.to_string()).await?;
    Ok(web::Json(token.to_string()))
}
