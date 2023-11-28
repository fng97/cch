use crate::helpers::spawn_app;

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
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(500, response.status().as_u16());
}
