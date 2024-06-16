use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;
use crate::domain::models::user::{CreateUser, User};
use crate::infrastructure::schema::users;

#[derive(Queryable)]
pub struct UserDiesel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password:String,
    pub created_at: DateTime<Utc>,
}

// Factory method for creating a new UserDiesel from a User
impl From<User> for UserDiesel {
    fn from(t: User) -> Self {
        UserDiesel {
            id: t.id,
            username: t.username,
            email: t.email,
            created_at: t.created_at,
            password:t.password,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel {
    pub username: String,
    pub email: String,
    pub password:String,
}

// Factory method for creating a new User from a UserDiesel
impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
            password: self.password,
            created_at:self.created_at,
        }
    }
}

impl From<CreateUser> for CreateUserDiesel {
    fn from(t: CreateUser) -> Self {
        CreateUserDiesel {
            username: t.username,
            email:t.email,
            password: t.password,
        }
    }
}

impl Into<User> for CreateUserDiesel {
    fn into(self) -> User {
        User {
            id: 0,
            username: self.username,
            email: self.email,
            password: self.password,
            created_at:chrono::Utc::now(),
        }
    }
}