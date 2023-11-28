use crate::helpers::spawn_app;

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
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("Hello, world!", response.text().await.unwrap());
}
