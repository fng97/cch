use crate::helpers::spawn_app;
use reqwest::Client;

// Task 1
#[tokio::test]
async fn gets_pokemon_weight() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{app_address}/8/weight/25"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("6", response.text().await.unwrap());
}
