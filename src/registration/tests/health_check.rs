use std::collections::HashMap;

#[cfg(test)]
pub mod utils {
    use registration::run;
    use tokio::net::TcpListener;

    pub async fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind address");
        let port = listener.local_addr().unwrap().port();

        let server = run(listener).await.expect("Failed to bind address");
        let _ = tokio::spawn(async { server.await.unwrap() });
        format!("http://127.0.0.1:{}", port)
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = utils::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = utils::spawn_app().await;
    let client = reqwest::Client::new();

    let mut form_data = HashMap::new();
    form_data.insert("name", "hello");
    form_data.insert("email", "world@world.com");

    let response = client
        .post(&format!("{}/subscribe", &address))
        .json(&form_data)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = utils::spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        HashMap::from([("email", "world@world.com")]),
        HashMap::from([("name", "world")]),
        HashMap::new()
    ];

    for form in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &address))
            .json(&form)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), "failed!");
    }

}
