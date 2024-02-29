use fake::{faker, Fake};
use sqlx::{Connection, PgConnection};

use crate::utils::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let config = zero2prod::config::Config::new();
    let mut connection = PgConnection::connect(&config.db_config.url)
        .await
        .expect("Failed to connect to Postgres.");

    let app_address = spawn_app().await.unwrap();

    let client = reqwest::Client::new();

    let name: String = faker::name::en::FirstName().fake();
    let email: String = faker::internet::en::SafeEmail().fake();

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
            error_message: "malformed email",
        },
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
