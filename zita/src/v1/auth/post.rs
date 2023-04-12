//! **POST** `/v1/auth` route handler.

use actix_web::web;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use deadpool_postgres::Pool;
use jsonwebtoken::EncodingKey;
use paperclip::actix::{api_v2_operation, post, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{
    identity::{Identity, Scopes},
    Error, Result,
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
    request: web::Json<LoginRequest>,
    key: web::Data<EncodingKey>,
    pool: web::Data<Pool>,
    argon2: web::Data<Argon2<'static>>,
) -> Result<web::Json<LoginResponse>> {
    let row = {
        let connection = pool.get().await?;

        connection
            .query_opt(
                r#"
        SELECT user_id, password
        FROM users
        WHERE users.email_address = $1
        "#,
                &[&request.username],
            )
            .await?
            .ok_or(Error::Authentication)?
    };

    let (id, hash): (uuid::Uuid, String) = (row.get(0), row.get(1));
    let hash =
        PasswordHash::new(&hash).map_err(|err| Error::InternalError(err.to_string().into()))?;

    match argon2.verify_password(request.password.as_bytes(), &hash) {
        Ok(_) => {
            let identity =
                Identity::new(id.hyphenated().to_string(), Scopes::default(), DEFAULT_TTL);

            Ok(web::Json(LoginResponse {
                access: identity
                    .encode(&key)
                    .map_err(|err| Error::InternalError(err.to_string().into()))?,
            }))
        }
        Err(_) => Err(Error::Authentication),
    }
}
