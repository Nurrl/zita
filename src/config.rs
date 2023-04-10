//! Types and data-structures project **configuration**.

use std::net::SocketAddr;

use deadpool_postgres::tokio_postgres::Config as PostgresConfig;
use envconfig::Envconfig;

/// The configuration of the project, loadable from env.
#[derive(Debug, Envconfig)]
pub struct Config {
    /// The address/port couple to bind for the server.
    #[envconfig(default = "0.0.0.0:8989")]
    pub http_address: SocketAddr,

    /// The connection URL to the postgres server.
    pub postgres_url: PostgresConfig,

    /// The postgres maximum connection count.
    #[envconfig(default = "64")]
    pub postgres_max_connections: usize,

    /// The EdDSA public key PEM for the JWT verification.
    pub jwt_public_key: String,

    /// The EdDSA private key PEM for the JWT signature.
    pub jwt_private_key: String,
}
