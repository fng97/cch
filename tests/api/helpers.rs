use actix_web::{App, HttpServer};
use std::net::TcpListener;

use cch::routing::config;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = HttpServer::new(|| App::new().configure(config))
        .listen(listener)
        .expect("Failed to bind TCP listener.")
        .run();

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
