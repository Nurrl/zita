//! **POST** `/v1/auth` route handler.

use actix_web::web;
use jsonwebtoken::EncodingKey;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{
    identity::{Identity, Scopes},
    Result,
};

/// The default applied TTL for issued tokens.
const DEFAULT_TTL: u64 = 3600; // 1h

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct LoginResponse {
    access: String,
}

/// Try to log-in a user from a `username` & `password` couple,
/// returning an `access` token to be passed as a `Bearer` in the `Authorization` header.
#[api_v2_operation(tags(auth))]
#[post("/auth")]
pub async fn route(
    _request: web::Json<LoginRequest>,
    key: web::Data<EncodingKey>,
) -> Result<web::Json<LoginResponse>> {
    let identity = Identity::new("to be filled :)".into(), Scopes::default(), DEFAULT_TTL);

    Ok(web::Json(LoginResponse {
        access: identity.encode(&key)?,
    }))
}
