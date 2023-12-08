use crate::helpers::spawn_app;

// Task 1
#[tokio::test]
async fn get_2_strengths_sums_reindeer_strengths() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(format!("{app_address}/4/strength"))
        .header("Content-Type", "application/json")
        .body(
            r#"
            [
                { "name": "Dasher", "strength": 5 },
                { "name": "Dancer", "strength": 6 },
                { "name": "Prancer", "strength": 4 },
                { "name": "Vixen", "strength": 7 }
            ]
            "#,
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("22", response.text().await.unwrap());
}
