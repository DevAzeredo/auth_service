use bcrypt::BcryptError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl From<BcryptError> for CommonError {
    fn from(error: BcryptError) -> Self {
        CommonError {
            message: format!("Bcrypt error: {}", error),
            code: 500,
        }
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}
impl From<jsonwebtoken::errors::Error> for CommonError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        CommonError {
            message: format!("JWT error: {}", error),
            code: 500,
        }
    }
}
