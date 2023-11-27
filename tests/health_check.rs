use actix_web::{App, HttpServer};

use cch::config;

fn spawn_app() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8000")?
        .run();

    tokio::spawn(server);

    Ok(())
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
