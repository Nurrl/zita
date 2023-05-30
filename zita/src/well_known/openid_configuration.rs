//! Implementation of the `/.well-known/openid-configuration` standard route.

use actix_web::web;

use paperclip::actix::{api_v2_operation, get};
use serde_json::Value;

use crate::{Error, Result};

#[api_v2_operation(tags(well-known))]
#[get("/openid-configuration")]
async fn get() -> Result<web::Json<Value>> {
    let metadata = serde_json::Value::default();

    Ok(web::Json(serde_json::to_value(metadata).map_err(
        |err| Error::InternalError(err.to_string().into()),
    )?))
}
