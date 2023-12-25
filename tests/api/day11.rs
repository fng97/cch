use crate::helpers::spawn_app;
use reqwest::Client;

// Task 1
#[tokio::test]
async fn returns_baubles_asset() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{app_address}/11/assets/decoration.png"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("image/png", response.headers()["content-type"]);
    assert_eq!(787297, response.content_length().unwrap());
}
