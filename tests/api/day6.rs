use crate::helpers::spawn_app;
use reqwest::{header, Client};
use serde_json::{from_str, Value};

// TODO: Add tests for the following
// - can match overlapping substrings
// - is case sensitive

#[tokio::test]
async fn counts_elves_and_shelves() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    let test_cases = [
        (
            "there is an elf on a shelf on an elf.
            there is also another shelf in Belfast.",
            r#"{
                "elf": 5,
                "elf on a shelf": 1,
                "shelf with no elf on it": 1
            }"#,
        ),
        (
            "there is an elf on a shelf on an elf.
            there is also another shelf in Belfast.",
            r#"{
                "elf": 5,
                "elf on a shelf": 1,
                "shelf with no elf on it": 1
            }"#,
        ),
    ];

    for (request_body, expected_response_body) in test_cases {
        // Act
        let response = client
            .post(format!("{app_address}/6"))
            .header(header::CONTENT_TYPE, "text/plain")
            .body(request_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(200, response.status().as_u16());

        let expected_body: Value = from_str(expected_response_body).unwrap();
        let actual_body: Value = from_str(&response.text().await.unwrap()).unwrap();

        assert_eq!(expected_body, actual_body);
    }
}
