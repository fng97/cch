use crate::helpers::spawn_app;
use reqwest::Client;
use serde_json::{from_str, Value};

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

// Task 2
#[tokio::test]
async fn counts_possible_bakes() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .get(format!("{app_address}/7/bake"))
        .header(
            "Cookie",
            "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let expected_body: Value = from_str(
        r#"{
        "cookies": 4,
        "pantry": {
            "flour": 5,
            "sugar": 307,
            "butter": 2002,
            "baking powder": 825,
            "chocolate chips": 257
        }
    }"#,
    )
    .unwrap();

    let actual_body: Value = from_str(&response.text().await.unwrap()).unwrap();

    assert_eq!(expected_body, actual_body);
}
