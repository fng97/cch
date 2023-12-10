use actix_web::{App, HttpServer};
use once_cell::sync::Lazy;
use std::net::TcpListener;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use cch::routing::config;

// ensure that the `tracing` stack is only initialised once
// FIXME: not getting anything out of actix-web, including instrumented routes
static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .init();
    };
});

pub async fn spawn_app() -> String {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = HttpServer::new(|| App::new().configure(config))
        .listen(listener)
        .expect("Failed to bind TCP listener.")
        .run();

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
