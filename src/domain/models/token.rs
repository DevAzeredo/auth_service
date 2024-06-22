use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub exp: i64,
}
