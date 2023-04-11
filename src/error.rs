//! Types and data-structures related to **error** management.

use std::borrow::Cow;

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

    #[error(r"Unable to authenticate your request with the provided parameters (・⌓・｀)")]
    Authentication,

    #[error("An internal error occured: {0}")]
    InternalError(Cow<'static, str>),

    /* Automatic errors; */
    #[error("Unable to get a connection from the database pool: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Error while communicating with the database: {0}")]
    DatabaseError(#[from] deadpool_postgres::tokio_postgres::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::NotFound => http::StatusCode::NOT_FOUND,
            Self::Unauthorized => http::StatusCode::UNAUTHORIZED,
            Self::Authentication => http::StatusCode::UNAUTHORIZED,
            Self::InternalError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,

            Self::PoolError(_) => http::StatusCode::SERVICE_UNAVAILABLE,
            Self::DatabaseError(_) => http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
