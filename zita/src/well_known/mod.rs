//! Implementations of the `/.well-known` routes for the SSO.

use paperclip::actix::web;

pub mod jwks;
pub mod openid_configuration;

/// List the services of the `/.well-known` sub-path.
pub fn mounts() -> web::Scope {
    web::scope("/.well-known")
        .service(openid_configuration::get)
        .service(jwks::get)
}
