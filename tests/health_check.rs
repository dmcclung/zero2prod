//! tests/healh_check.rs

use std::net::TcpListener;
use sqlx::{Connection, PgConnection };
use url::form_urlencoded;

#[tokio::test]
async fn health_check_ok() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client.get(&format!("{}/health_check", &app_address))
        .send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let config = zero2prod::config::Config::new();
    let mut connection = PgConnection::connect(&config.db_config.url)
        .await
        .expect("Failed to connect to Postgres.");
    
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let email: String = form_urlencoded::byte_serialize("joemayo@zero2prod.com".as_bytes()).collect();
    let name = "joe";

    let body = format!("name={}&email={}", name, email);
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, email);
    assert_eq!(saved.name, name);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    struct TestCase<'a> {
        body: &'a str,        
        error_message: &'a str
    }

    let test_cases = vec![
        TestCase { body: "", error_message: "missing name and email" },
        TestCase { body: "email=joemayo%40gmail.com", error_message: "missing name" },
        TestCase { body: "name=joe", error_message: "missing email" }
    ];

    for test_case in test_cases {
        
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(test_case.body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400, 
            response.status().as_u16(), 
            "The API did not fail with 400 Bad Request {}.", 
            test_case.error_message);
    }
}