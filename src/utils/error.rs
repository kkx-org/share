use crate::utils::validator::ValidationErrors;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    #[error("API validation error: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Argon2 error: {0}")]
    Argon2(#[from] argon2::Error),
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ApiError {
    pub status_code: StatusCode,
    pub kind: String,
    pub message: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiErrorResponse<'a> {
    Message {
        code: String,
        message: String,
    },
    Validation {
        code: String,
        errors: &'a ValidationErrors,
    },
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match &self {
            CustomError::Api(err) => err.status_code,
            CustomError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(match &self {
            CustomError::Api(err) => ApiErrorResponse::Message {
                code: err.kind.clone(),
                message: err.to_string(),
            },
            CustomError::Validation(err) => ApiErrorResponse::Validation {
                code: String::from("validation_error"),
                errors: err,
            },
            _ => ApiErrorResponse::Message {
                code: String::from("internal_server_error"),
                message: String::from("Internal Server Error"),
            },
        })
    }
}
