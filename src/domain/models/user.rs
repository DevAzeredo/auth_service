use chrono::{DateTime, Utc};
use serde::Deserialize;
#[derive(Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email:String,
    pub created_at: DateTime<Utc>,
}
#[derive(Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email:String,
}
