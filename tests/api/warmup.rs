use crate::helpers::spawn_app;

// Task 1
#[tokio::test]
async fn root_responds_with_hello_world() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&app_address)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("Hello, world!", response.text().await.unwrap());
}

// Task 2
#[tokio::test]
async fn fake_error_returns_500() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/-1/error", &app_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(500, response.status().as_u16());
}
