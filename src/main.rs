use color_eyre::eyre::{self, Context};
use envconfig::Envconfig;

use actix_web::{App, HttpServer, ResponseError};
use deadpool_postgres::tokio_postgres;
use jsonwebtoken::{DecodingKey, EncodingKey};
use paperclip::actix::{web, OpenApiExt};

mod config;
mod error;
mod identity;

mod probes;
mod v1;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set-up the pretty-printed error handler
    color_eyre::install()?;

    // Set-up the log and traces handler
    tracing_subscriber::fmt().init();

    // Load the config from the environment
    let config = config::Config::init_from_env()?;

    let jwt_decoding_key = web::Data::new(
        DecodingKey::from_ed_pem(config.jwt_public_key.as_bytes())
            .wrap_err("While reading the JWT_PUBLIC_KEY")?,
    );
    let jwt_encoding_key = web::Data::new(
        EncodingKey::from_ed_pem(config.jwt_private_key.as_bytes())
            .wrap_err("While reading the JWT_PRIVATE_KEY")?,
    );

    let postgres_pool = web::Data::new(
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            config.postgres_url,
            tokio_postgres::NoTls,
        ))
        .max_size(config.postgres_max_connections)
        .build()
        .wrap_err("While building the Postgres connection Pool")?,
    );

    HttpServer::new(move || {
        let app = App::new()
            .wrap_api()
            .app_data(jwt_decoding_key.clone())
            .app_data(jwt_encoding_key.clone())
            .app_data(postgres_pool.clone())
            .service(probes::healthz)
            .service(v1::mounts())
            .default_service(web::to(|| async { Error::NotFound.error_response() }));

        // Add the swagger endpoints when compiling in debug mode
        #[cfg(debug_assertions)]
        let app = app
            .with_json_spec_at("/swagger/spec/v2")
            .with_swagger_ui_at("/swagger");

        app.build()
    })
    .bind(config.http_address)?
    .run()
    .await?;

    Ok(())
}
