//! All routes related to project liveness probing.

use actix_web::web;
use paperclip::actix::{api_v2_operation, get, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct HealthInfo {
    version: &'static str,
}

/// A health-check endpoint to know whenever the server is ready
/// and implement uptime monitoring.
#[api_v2_operation(tags(probes))]
#[get("/healthz")]
async fn healthz() -> web::Json<HealthInfo> {
    web::Json(HealthInfo {
        version: env!("CARGO_PKG_VERSION"),
    })
}
