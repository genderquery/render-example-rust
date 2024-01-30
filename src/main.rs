use std::env;

use render_example::app;
use tracing::debug;
use tracing_subscriber::prelude::*;

struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn try_from_env() -> Result<Config, anyhow::Error> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("PORT").unwrap_or_else(|_| "3000".into()).parse()?;
        Ok(Config { host, port })
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "render_example=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::try_from_env().unwrap();
    let listener = tokio::net::TcpListener::bind((config.host, config.port))
        .await
        .unwrap();
    debug!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}
