use crate::helpers::spawn_app;
use reqwest::Client;

// Task 1
#[tokio::test]
async fn gets_pokemon_weight() {
    // Arrange
    let app_address = spawn_app();
    let client = Client::new();

    let test_cases = [
        (25, 6.),
        (393, 5.2), // support floats
    ];

    for (pokedex, expected_weight) in test_cases {
        // Act
        let response = client
            .get(format!("{app_address}/8/weight/{pokedex}"))
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(200, response.status().as_u16());
        assert_eq!(expected_weight.to_string(), response.text().await.unwrap());
    }
}
