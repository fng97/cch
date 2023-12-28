use crate::helpers::spawn_app;
use reqwest::Client;

// TODO: Find a way to mock the time (on per-thread basis:
// https://ditto.live/blog/mocking-time-in-async-rust?utm_source=reddit-rust&utm_campaign=async-rust-blog)

// Task 1
#[tokio::test]
async fn measures_interval_between_save_and_load() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let _ = client
        .post(format!("{app_address}/12/save/packet20231212"))
        .send()
        .await
        .expect("Failed to execute request");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let response = client
        .get(format!("{app_address}/12/load/packet20231212"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("1", response.text().await.unwrap());
}
