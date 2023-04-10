//! Types and data-structures related to **error** management.

use actix_web::{http, ResponseError};
use paperclip::actix::api_v2_errors;
use thiserror::Error;

/// A handy type alias with the [`enum@Error`] by default.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A set of all errors that could end up happening in the application.
#[api_v2_errors]
#[derive(Debug, Error)]
pub enum Error {
    #[error(r"The requested resource was not found server-side ¯\_(ツ)_/¯")]
    NotFound,

    #[error(r"The requested resource requires authentication ಠ‿↼")]
    Unauthorized,

    #[error("Failed to encode or decode a JWT: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::NotFound => http::StatusCode::NOT_FOUND,
            Self::Unauthorized => http::StatusCode::UNAUTHORIZED,
            Self::JWTError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
