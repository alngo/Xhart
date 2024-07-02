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
        .post(&format!("{}/subscriptions", &address))
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

    //TODO: Add test_cases

    let mut form_data = HashMap::new();
    form_data.insert("email", "world@world.com");

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .json(&form_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // TODO: Add custom error message
    assert_eq!(400, response.status().as_u16());
}
