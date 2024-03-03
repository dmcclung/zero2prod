use claims::assert_ok;
use fake::{faker, Fake};

use crate::utils::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await.unwrap();
    assert_ok!(test_app.reset_subscriptions().await);

    let client = reqwest::Client::new();

    let name: String = faker::name::en::FirstName().fake();
    let email: String = faker::internet::en::SafeEmail().fake();

    let body = format!("name={}&email={}", name, email);

    let response = client
        .post(&format!("{}/subscriptions", &test_app.address()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let subscription = test_app.get_subscription().await;
    assert_eq!(subscription.0, email);
    assert_eq!(subscription.1, name);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await.unwrap();
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
            .post(&format!("{}/subscriptions", test_app.address()))
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
