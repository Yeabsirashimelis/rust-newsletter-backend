// actix_web::test - is the testing equivalent of actix_web::main
//                  it also spares you from having to specify the #[test] attribute

use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    //   Arrange
    let address = spawn_app().await;

    // we need to bring reqwest (like axios for JS) - we don't use actix b/c we want our tests to be platfrom independent
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

// spawn app is the only piece that will, resonably, depend on our application code
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    //  we retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    // initialize the actix server here for testing
    let server = newsletter_backend::run(listener).expect("Failed to bind address");

    // launch the server as a backgound task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non_binding let
    let _ = tokio::spawn(server); // MEANS - START MY WEB SERVER, BUT DON'T BLOCK THIS FUNCTION - KEEP GOING WITH THE REST OF THE CODE

    // we return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
