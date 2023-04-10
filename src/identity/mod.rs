//! Data-structures related to user **identity** and **scope management**.

use actix_web::{dev::Payload, http, web, FromRequest, HttpRequest};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use paperclip::actix::Apiv2Security;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

mod scope;
pub use scope::{Scope, Scopes};

/// The [`Algorithm`] to encode and decode the JWTs in the app.
pub const ALGORITHM: Algorithm = Algorithm::EdDSA;

// / A representation of the identity of the current requesting client.
#[derive(Debug, Serialize, Deserialize, Apiv2Security)]
#[openapi(
    apiKey,
    in = "header",
    name = "Authorization",
    description = "Enter the token with the `Bearer ` prefix, e.g. \"Bearer abcde12345\"."
)]
pub struct Identity {
    sub: String,
    iat: u64,
    nbf: u64,
    exp: u64,
    scope: Scopes,
}

impl Identity {
    /// Create a new [`Identity`] for the provided `subject` with
    /// the provided `scopes` expiring in `ttl` seconds.
    pub fn new(subject: String, scopes: Scopes, ttl: u64) -> Self {
        let now = jsonwebtoken::get_current_timestamp();

        Self {
            sub: subject,
            iat: now,
            nbf: now,
            exp: now + ttl,
            scope: scopes,
        }
    }

    /// Get a reference the [`Identity`]'s `sub` field.
    pub fn subject(&self) -> &str {
        &self.sub
    }

    /// Get a reference the [`Identity`]'s `scope` field.
    pub fn scopes(&self) -> &Scopes {
        &self.scope
    }

    /// Decode an [`Identity`] from a JWT using the provided [`DecodingKey`].
    pub fn decode(key: &DecodingKey, value: impl AsRef<str>) -> Result<Self> {
        let mut validation = jsonwebtoken::Validation::default();

        validation.algorithms = vec![ALGORITHM];
        validation.validate_exp = true;
        validation.validate_nbf = true;

        Ok(jsonwebtoken::decode(value.as_ref(), key, &validation)?.claims)
    }

    /// Encode the [`Identity`] to a JWT using the provided [`EncodingKey`].
    pub fn encode(&self, key: &EncodingKey) -> Result<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header {
                alg: ALGORITHM,
                ..Default::default()
            },
            self,
            key,
        )
        .map_err(Into::into)
    }
}

impl FromRequest for Identity {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let parse = || {
            let key = req
                .app_data::<web::Data<DecodingKey>>()
                .expect("The `jsonwebtoken::DecodingKey` hasn't been initialized before.");

            let bearer = req
                .headers()
                .get(http::header::AUTHORIZATION)
                .filter(|header| header.len() >= 8)
                .map(http::header::HeaderValue::to_str)
                .and_then(Result::ok)
                .and_then(|header| match header.split_once(' ') {
                    Some(("Bearer", value)) => Some(value),
                    _ => None,
                })
                .ok_or(Error::Unauthorized)?;

            Self::decode(key, bearer).map_err(|_| Error::Unauthorized)

            // TODO: Verify if the user is still active in the database
        };

        futures::future::ready(parse())
    }
}
