use std::net::TcpListener;

use actix_web::dev::Server;
use reqwest;
use tokio;
use zero2prod::run;
use zero2prod::test_utils::helpers::spawn_app;

#[actix_rt::test]
async fn sign_up_for_newletter_works_when_name_and_email_supplied() {
    // Arrange
    let tcp_port = spawn_app(); // hmm we probably don't want to do this every time.
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
    let tcp_port = spawn_app(); // hmm we probably don't want to do this every time.
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
    let tcp_port = spawn_app(); // hmm we probably don't want to do this every time.
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
