//! All `/v1` routes of the APIs.

use paperclip::actix::web;

pub mod auth;

/// List the services of the `/v1` sub-path.
pub fn mounts() -> web::Scope {
    web::scope("/v1")
        .service(auth::post::route)
        .service(auth::get::route)
}
