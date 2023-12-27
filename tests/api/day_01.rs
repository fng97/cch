use crate::helpers::spawn_app;

#[tokio::test]
async fn cubes_xored_path_params() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // example inputs
    let test_cases = [
        ("4/8", 200, "1728"),     // GET /1/4/8 returns 1728
        ("10", 200, "1000"),      // GET /1/10 returns 10
        ("4/5/8/10", 200, "27"),  // GET /1/4/5/8/10 returns 27
        ("", 400, "Bad Request"), // no path params
        (
            "1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/17/18/19/20/21",
            400,
            "Bad Request",
        ), // too many path params
        ("4/-8", 200, "-64"),     // support negative numbers
    ];

    for (input, expected_status, expected_body) in test_cases {
        // Act
        let response = client
            // from example: xor(4, 8)^3 = 1728
            .get(format!("{app_address}/1/{input}"))
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(expected_status, response.status().as_u16());
        assert_eq!(expected_body, response.text().await.unwrap());
    }
}
