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

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missingtheemail"),
        ("email=ursula_le_guin%40gmail.com", "missingthename"),
        ("", "missing bothname and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
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
