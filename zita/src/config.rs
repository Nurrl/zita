//! Types and data-structures project **configuration**.

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use aliri::{
    jwa::{self, ec, rsa},
    jws, Jwk, Jwks,
};
use color_eyre::eyre::{self, Context};
use deadpool_postgres::tokio_postgres::Config as PostgresConfig;
use envconfig::Envconfig;
use serde::{Deserialize, Serialize};
use sha1::Digest;

/// The environment configuration variables structure.
#[derive(Debug, Envconfig)]
pub struct Env {
    /// The path to the configuration file of the project.
    #[envconfig(default = "./config.yaml")]
    pub configuration_path: PathBuf,

    /// The connection URL to the postgres server.
    pub postgres_url: PostgresConfig,

    /// The postgres maximum connection count.
    #[envconfig(default = "64")]
    pub postgres_max_connections: usize,
}

impl Env {
    /// Load the configuration file from the provided path.
    pub async fn load_config(&self) -> eyre::Result<Config> {
        let conf = tokio::fs::read(&self.configuration_path)
            .await
            .wrap_err("While reading configuration file")?;

        serde_yaml::from_slice(&conf).wrap_err("While deserializing configuration")
    }
}

/// The file-based configuration for the project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    http: HttpConfig,
    #[serde(default)]
    argon2: Argon2Config,
    keys: Vec<KeyConfig>,
}

impl Config {
    /// The default HTTP address for the API.
    pub const DEFAULT_HTTP_ADDRESS: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8989);

    /// Construct the [`SocketAddr`] from the configuration.
    pub fn to_socketaddr(&self) -> SocketAddr {
        self.http.address.unwrap_or(Self::DEFAULT_HTTP_ADDRESS)
    }

    /// Construct a new [`argon2::Argon2`] instance from configuration.
    pub fn to_argon2<'a>(&self) -> eyre::Result<argon2::Argon2<'a>> {
        let params = argon2::Params::new(
            self.argon2
                .memory_cost
                .unwrap_or(argon2::Params::DEFAULT_M_COST),
            self.argon2
                .time_cost
                .unwrap_or(argon2::Params::DEFAULT_T_COST),
            self.argon2
                .paralellism_cost
                .unwrap_or(argon2::Params::DEFAULT_P_COST),
            self.argon2.output_len,
        )
        .wrap_err("While configuring the Argon2 parameters")?;

        Ok(argon2::Argon2::new(
            argon2::Algorithm::default(),
            argon2::Version::default(),
            params,
        ))
    }

    /// Construct a [`aliri::Jwks`] from the provided keys.
    pub async fn to_jwks(&self) -> eyre::Result<Jwks> {
        let mut jwks = Jwks::default();

        for key in self.keys.iter() {
            let pem = tokio::fs::read_to_string(&key.path).await?;

            let jwk = match &key.algorithm {
                jws::Algorithm::Rsa(_) => Jwk::from(rsa::PrivateKey::from_pem(&pem)?),
                jws::Algorithm::EllipticCurve(_) => Jwk::from(ec::PrivateKey::from_pem(&pem)?),
                _ => {
                    return Err(eyre::eyre!(
                        "Usage of symetric signing algorithm is not supported ({})",
                        key.algorithm
                    ))
                }
            };

            let kid = sha1::Sha1::digest(pem)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>();

            jwks.add_key(
                jwk.with_usage(jwa::Usage::Signing)
                    .with_algorithm(key.algorithm)
                    .with_key_id(kid.into()),
            );
        }

        Ok(jwks)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HttpConfig {
    /// The address/port couple to bind for the HTTP server.
    address: Option<SocketAddr>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Argon2Config {
    /// The memory complexity of the Argon2 hashes to be created.
    memory_cost: Option<u32>,

    /// The time complexity of the Argon2 hashes to be created.
    time_cost: Option<u32>,

    /// The paralellism complexity of the Argon2 hashes to be created.
    paralellism_cost: Option<u32>,

    /// The length in bytes of the Argon2 hashes to be created.
    output_len: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    /// The algorithm of the speficied key path.
    algorithm: jws::Algorithm,

    /// The path of the PEM-encoded private key file.
    path: PathBuf,
}
