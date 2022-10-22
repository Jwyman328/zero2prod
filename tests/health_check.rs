use std::net::TcpListener;

use actix_web::dev::Server;
use reqwest;
use tokio;
use zero2prod::run;

// 'actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
// USe `cargo add actix-rt --dev --vers 2` to add `actix-rt`

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let test_server_address = "127.0.0.1:0"; // port 0 tells the OS go find me an available port.
    let tcp_port = spawn_app(&test_server_address); // spin up server in other thread
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

#[actix_rt::test]
async fn sign_up_for_newletter_works_when_name_and_email_supplied() {
    // Arrange
    let test_server_address = "127.0.0.1:0";
    let tcp_port = spawn_app(&test_server_address); // hmm we probably don't want to do this every time.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(format!("http://{}/sign_up_for_newsletter", tcp_port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn sign_up_returns_400_when_name_not_supplied() {
    // Arrange
    let test_server_address = "127.0.0.1:0";
    let tcp_port = spawn_app(&test_server_address); // hmm we probably don't want to do this every time.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(format!("http://{}/sign_up_for_newsletter", tcp_port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 400);
}

#[actix_rt::test]
async fn sign_up_returns_400_when_email_not_supplied() {
    // Arrange
    let test_server_address = "127.0.0.1:0";
    let tcp_port = spawn_app(&test_server_address); // hmm we probably don't want to do this every time.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .post(format!("http://{}/sign_up_for_newsletter", tcp_port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin")
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 400);
}

fn spawn_app(test_server_address: &str) -> std::net::SocketAddr {
    // pass in random port
    let tcp_listener =
        TcpListener::bind(test_server_address).expect("tcp listener spawn app failed");
    let tcp_port = tcp_listener
        .local_addr()
        .expect("unwrap local addr")
        .clone();

    let server = run(tcp_listener).expect("error starting server");
    let _ = tokio::spawn(server);
    tcp_port
}
