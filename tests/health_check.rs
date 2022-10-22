use reqwest;
use zero2prod::test_utils::helpers::spawn_app;

// 'actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
// USe `cargo add actix-rt --dev --vers 2` to add `actix-rt`

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
     // port 0 tells the OS go find me an available port.
    let tcp_port = spawn_app(); // spin up server in other thread
                                                    // We need to bring in `request`
                                                    // to perform HTTP request against our app
                                                    //
                                                    // Use cargo add reqwest --dev
                                                    //
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(format!("http://{}/health_check", tcp_port))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
