use registration::run;

async fn spawn_app() {
    let server = run().await.expect("Failed to bind address");
    let _ = tokio::spawn(async { server.await.unwrap() });
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
