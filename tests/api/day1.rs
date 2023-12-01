use crate::helpers::spawn_app;

// Task 1
#[tokio::test]
async fn get_1_cubes_xored_path_param_pair() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let challenge_path = format!("{}/1", &app_address);

    // Act
    let response = client
        // from example: xor(4, 8)^3 = 1728
        .get(&format!("{challenge_path}/4/8"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("1728", response.text().await.unwrap());
}
