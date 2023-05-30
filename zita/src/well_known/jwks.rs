//! Implementation of the `/.well-known/jwks.json` standard route.

use actix_web::web;

use paperclip::actix::{api_v2_operation, get, Apiv2Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Jwks {
    keys: Vec<Value>,
}

#[api_v2_operation(tags(well-known))]
#[get("/jwks.json")]
async fn get(jwks: web::Data<aliri::Jwks>) -> Result<web::Json<Jwks>> {
    let jwks = Jwks {
        keys: jwks
            .keys()
            .iter()
            .cloned()
            .map(aliri::Jwk::public_only)
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()?,
    };

    Ok(web::Json(jwks))
}
