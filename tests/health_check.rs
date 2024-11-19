
#[actix_web::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // bring in reqwest
    // to perform HTTP requests against app
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch server as background task
    let _ = actix_web::rt::spawn(server);
}
