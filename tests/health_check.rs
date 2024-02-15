//! tests/healh_check.rs

use sqlx::{postgres::PgPoolOptions, Connection, PgConnection};
use std::net::TcpListener;
use url::form_urlencoded;

#[tokio::test]
async fn health_check_ok() {
    let app_address = spawn_app().await.unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<String, sqlx::Error> {
    let config = zero2prod::config::Config::new();
    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::migrate!().run(&pool).await?;
    sqlx::query!("DELETE FROM subscriptions")
        .execute(&pool)
        .await?;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener, pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    Ok(format!("http://127.0.0.1:{}", port))
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let config = zero2prod::config::Config::new();
    let mut connection = PgConnection::connect(&config.db_config.url)
        .await
        .expect("Failed to connect to Postgres.");

    let app_address = spawn_app().await.unwrap();

    let client = reqwest::Client::new();

    let email = "dev@zero2prod.xyz";
    let name = "dev";

    let body = format!(
        "name={}&email={}",
        form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>(),
        form_urlencoded::byte_serialize(email.as_bytes()).collect::<String>()
    );
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
    let app_address = spawn_app().await.unwrap();
    let client = reqwest::Client::new();

    struct TestCase<'a> {
        body: &'a str,
        error_message: &'a str,
    }

    let test_cases = vec![
        TestCase {
            body: "",
            error_message: "missing name and email",
        },
        TestCase {
            body: "email=dev%40zero2prod.xyz",
            error_message: "missing name",
        },
        TestCase {
            body: "name=dev",
            error_message: "missing email",
        },
        TestCase {
            body: "name=user&email=no-at-sign.com",
            error_message: "malformed email"
        }
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
            test_case.error_message
        );
    }
}
