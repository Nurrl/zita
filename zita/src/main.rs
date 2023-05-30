use actix_web::{App, HttpServer, ResponseError};
use color_eyre::eyre::{self, Context};
use deadpool_postgres::tokio_postgres;
use envconfig::Envconfig;
use paperclip::actix::{web, OpenApiExt};

mod config;
mod error;
// mod identity;

mod probes;
// mod v1;
mod well_known;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Set-up the pretty-printed error handler
    color_eyre::install()?;

    // Set-up the log and traces handler
    tracing_subscriber::fmt().init();

    // Load the config from the environment
    let env = config::Env::init_from_env()?;
    let config = env.load_config().await?;

    let pool = web::Data::new(
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            env.postgres_url,
            tokio_postgres::NoTls,
        ))
        .max_size(env.postgres_max_connections)
        .build()
        .wrap_err("While building the Postgres connection Pool")?,
    );

    let argon2 = web::Data::new(config.to_argon2()?);
    let jwks = web::Data::new(config.to_jwks().await?);

    HttpServer::new(move || {
        let app = App::new()
            .wrap_api()
            .app_data(pool.clone())
            .app_data(argon2.clone())
            .app_data(jwks.clone())
            // .app_data(keypairs.clone())
            .service(probes::healthz)
            .service(well_known::mounts())
            // .service(v1::mounts())
            .default_service(web::to(|| async { Error::NotFound.error_response() }));

        // Add the swagger endpoints when compiling in debug mode
        #[cfg(debug_assertions)]
        let app = app
            .with_json_spec_at("/swagger/spec/v2")
            .with_swagger_ui_at("/swagger");

        app.build()
    })
    .bind(config.to_socketaddr())?
    .run()
    .await?;

    Ok(())
}
