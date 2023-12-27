use crate::helpers::spawn_app;
use serde_json::{from_str, Value};

// Task 1
#[tokio::test]
async fn sums_reindeer_strengths() {
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
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("22", response.text().await.unwrap());
}

// Task 2
#[tokio::test]
async fn summarises_winners() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(format!("{app_address}/4/contest"))
        .header("Content-Type", "application/json")
        .body(
            r#"
            [
                {
                    "name": "Dasher",
                    "strength": 5,
                    "speed": 50.4,
                    "height": 80,
                    "antler_width": 36,
                    "snow_magic_power": 9001,
                    "favorite_food": "hay",
                    "cAnD13s_3ATeN-yesT3rdAy": 2
                },
                {
                    "name": "Dancer",
                    "strength": 6,
                    "speed": 48.2,
                    "height": 65,
                    "antler_width": 37,
                    "snow_magic_power": 4004,
                    "favorite_food": "grass",
                    "cAnD13s_3ATeN-yesT3rdAy": 5
                }
            ]
            "#,
        )
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let expected_body: Value = from_str(
        r#"
        {
            "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
            "tallest": "Dasher is standing tall with his 36 cm wide antlers",
            "magician": "Dasher could blast you away with a snow magic power of 9001",
            "consumer": "Dancer ate lots of candies, but also some grass"
        }
        "#,
    )
    .unwrap();
    let actual_body: Value = from_str(&response.text().await.unwrap()).unwrap();

    assert_eq!(expected_body, actual_body);
}
