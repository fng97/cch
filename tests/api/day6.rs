use crate::helpers::spawn_app;
use reqwest::{header, Client};
use serde_json::{from_str, Value};

// Task 1
#[tokio::test]
async fn counts_elf_sub_strings() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    // Act
    let response = client
        .post(format!("{app_address}/6"))
        .header(header::CONTENT_TYPE, "text/plain")
        .body(
            "The mischievous elf peeked out from behind the toy workshop,
        and another elf joined in the festive dance.
        Look, there is also an elf on that shelf!",
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let expected_body: Value = from_str(r#"{"elf":4}"#).unwrap();
    let actual_body: Value = from_str(&response.text().await.unwrap()).unwrap();

    assert_eq!(expected_body, actual_body);
}
