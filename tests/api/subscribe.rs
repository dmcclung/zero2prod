use fake::{faker, Fake};

use crate::test_app::spawn;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn().await.unwrap();

    let name: String = faker::name::en::FirstName().fake();
    let email: String = faker::internet::en::SafeEmail().fake();

    let response = test_app
        .create_subscription(name.clone(), email.clone())
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let subscriber_id = test_app.get_subscription(&name, &email).await;
    let subscription_token = test_app.get_subscription_token(subscriber_id).await;

    let expected_confirmation_link =
        &format!("https://zero2prod.xyz/confirm?token={}", subscription_token);

    let sent_messages = test_app.get_sent_emails();
    assert_eq!(sent_messages.len(), 1);
    assert_eq!(
        sent_messages[0].1.contains(expected_confirmation_link),
        true
    );
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn().await.unwrap();

    struct TestCase<'a> {
        name: &'a str,
        email: &'a str,
        error_message: &'a str,
    }

    let test_cases = vec![
        TestCase {
            name: "",
            email: "",
            error_message: "missing name and email",
        },
        TestCase {
            name: "",
            email: "dev%40zero2prod.xyz",
            error_message: "missing name",
        },
        TestCase {
            name: "dev",
            email: "",
            error_message: "missing email",
        },
        TestCase {
            name: "user",
            email: "no-at-sign.com",
            error_message: "malformed email",
        },
    ];

    for test_case in test_cases {
        let response = test_app
            .create_subscription(test_case.name.into(), test_case.email.into())
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
