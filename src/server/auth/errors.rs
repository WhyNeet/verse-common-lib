use std::collections::HashMap;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("An internal error occured.")]
    Internal,

    #[error("")]
    InvalidPayload(HashMap<&'static str, Option<String>>),

    #[error("User with this {0} already exists.")]
    UserAlreadyExists(&'static str),

    #[error("Invalid token provided.")]
    InvalidToken,

    #[error("Invalid credentials provided.")]
    InvalidCredentials,

    #[error("User with this {0} does not exist.")]
    UserDoesNotExist(&'static str),

    #[error("Wrong password.")]
    WrongPassword,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidPayload(_)
            | Self::UserAlreadyExists(_)
            | AuthError::InvalidToken
            | AuthError::InvalidCredentials
            | AuthError::WrongPassword
            | AuthError::UserDoesNotExist(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let response = match self {
            AuthError::InvalidPayload(payload) => json!({ "error": payload }),
            other => json!({ "error": other.to_string() }),
        };

        HttpResponse::build(self.status_code()).json(response)
    }
}

pub type AuthResult<T> = Result<T, AuthError>;
