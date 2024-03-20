use fake::{faker, Fake};

use crate::test_app::spawn;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn().await.unwrap();

    let name: String = faker::name::en::FirstName().fake();
    let email: String = faker::internet::en::SafeEmail().fake();

    let response = test_app
        .post_subscriptions(name.clone(), email.clone())
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let subscriber_id = test_app.get_subscription(&name, &email).await;
    let subscription_token = test_app.get_subscription_token(subscriber_id).await;

    assert_eq!(test_app.get_emails_sent(), 1);

    assert_eq!(
        test_app.email_body_contains(&format!(
            "https://zero2prod.xyz/con=\r\nfirm?token=3D{}",
            subscription_token
        )),
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
            .post_subscriptions(test_case.name.into(), test_case.email.into())
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
