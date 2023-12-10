use crate::helpers::spawn_app;
use reqwest::Client;

// Task 1
#[tokio::test]
async fn base64_decodes_recipe_from_cookie() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{app_address}/7/decode"))
        .header(
            "Cookie",
            "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(
        r#"{"flour":100,"chocolate chips":20}"#,
        response.text().await.unwrap()
    );
}
