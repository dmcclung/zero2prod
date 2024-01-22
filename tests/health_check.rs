//! tests/healh_check.rs

use std::net::TcpListener;
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
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let email: String = form_urlencoded::byte_serialize("joemayo@zero2prod.com".as_bytes()).collect();

    let body = format!("name=joe&email={}", email);    
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    struct TestCase<'a> {
        name: &'a str,
        email: &'a str,
        error_message: &'a str
    }

    let test_cases = vec![
        TestCase { name: "", email: "", error_message: "missing name and email" },
        TestCase { name: "", email: "joemayo@zero2prod.com", error_message: "missing name" },
        TestCase { name: "joe", email: "", error_message: "missing email" }
    ];

    for test_case in test_cases {
        let email_encoded: String = form_urlencoded::byte_serialize(test_case.email.as_bytes()).collect();
        let name_encoded: String = form_urlencoded::byte_serialize(test_case.name.as_bytes()).collect();

        let body = format!("name={}&email={}", name_encoded, email_encoded);
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
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