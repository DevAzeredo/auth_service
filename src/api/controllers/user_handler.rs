use actix_web::{web, Result};
use crate::api::dto::user::{CreateUserDTO, UserDTO};
use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>, post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<UserDTO>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(user.into()))
}

pub async fn get_user_handler(
    user_service: web::Data<dyn UserService>, params: web::Path<i32>,
) -> Result<web::Json<UserDTO>, ApiError> {
    let user = user_service.get(params.into_inner()).await?;
    Ok(web::Json(user.into()))
}

