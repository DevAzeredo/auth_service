use crate::domain::models::user::{CreateUser, LoginUser, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub email: String,
}
impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            username: self.username,
            password: self.password,
            email: self.email,
        }
    }
}

impl Into<CreateUserDTO> for CreateUser {
    fn into(self) -> CreateUserDTO {
        CreateUserDTO {
            username: self.username,
            password: self.password,
            email: self.email,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginUserDTO {
    pub username: String,
    pub password: String,
}

impl Into<LoginUser> for LoginUserDTO {
    fn into(self) -> LoginUser {
        LoginUser {
            username: self.username,
            password: self.password,
        }
    }
}
impl Into<LoginUserDTO> for LoginUser {
    fn into(self) -> LoginUserDTO {
        LoginUserDTO {
            username: self.username,
            password: self.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            created_at: self.created_at,
        }
    }
}
