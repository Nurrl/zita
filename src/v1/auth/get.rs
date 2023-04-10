//! **GET** `/v1/auth` route handler.

use actix_web::web;
use paperclip::actix::{api_v2_operation, get, Apiv2Schema};
use serde::{Deserialize, Serialize};

use crate::{
    identity::{Identity, Scopes},
    Result,
};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct InfoResponse {
    subject: String,
    scopes: Scopes,
}

/// Get a set of basic informations identifying the requester's session,
/// passed as a `Bearer` in the `Authorization` header.
#[api_v2_operation(tags(auth))]
#[get("/auth")]
pub async fn route(identity: Identity) -> Result<web::Json<InfoResponse>> {
    Ok(web::Json(InfoResponse {
        subject: identity.subject().into(),
        scopes: identity.scopes().clone(),
    }))
}
