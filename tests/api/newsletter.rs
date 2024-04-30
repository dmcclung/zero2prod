//! tests/api/newsletter.rs

use crate::test_app::spawn;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::en::FirstName;
use fake::Fake;

#[tokio::test]
async fn publish_newsletter_returns_200() {
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let test_app = spawn().await.unwrap();

    test_app
        .add_test_user("admin".to_string(), "password".to_string())
        .await;

    let response = test_app
        .publish_newsletter(
            Some(html),
            Some(text),
            Some(subject),
            "admin",
            Some("password"),
        )
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn missing_authorization_returns_401() {
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let test_app = spawn().await.unwrap();

    let response = test_app
        .publish_newsletter(Some(html), Some(text), Some(subject), "admin", None)
        .await
        .expect("Failed to post subscription");
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn bad_password_returns_401() {
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let test_app = spawn().await.unwrap();
    test_app
        .add_test_user("admin".to_string(), "password".to_string())
        .await;

    let response = test_app
        .publish_newsletter(
            Some(html),
            Some(text),
            Some(subject),
            "admin",
            Some("bad_pass"),
        )
        .await
        .expect("Failed to post subscription");
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn publish_newsletter_returns_400_with_bad_html_text() {
    let subject: String = Sentence(1..2).fake();
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));

    let test_cases = [
        (None, None, None, "Missing all fields"),
        (None, None, Some(subject.clone()), "Missing body"),
        (
            None,
            Some(text.clone()),
            Some(subject.clone()),
            "Missing html body",
        ),
        (
            None,
            Some(text.clone()),
            None,
            "Missing subject and html body",
        ),
        (Some(html.clone()), None, Some(subject), "Missing text body"),
        (
            Some(html.clone()),
            None,
            None,
            "Missing text body and subject",
        ),
        (Some(html), Some(text), None, "Missing subject"),
    ];

    let test_app = spawn().await.unwrap();

    for test_case in test_cases {
        let response = test_app
            .publish_newsletter(
                test_case.0,
                test_case.1,
                test_case.2,
                "admin",
                Some("password"),
            )
            .await
            .expect("Failed to post subscription");
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request, {}",
            test_case.3
        );
    }
}

#[tokio::test]
async fn newsletter_sent_to_confirmed_subscribers() {
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let test_app = spawn().await.unwrap();
    test_app
        .add_test_user("admin".to_string(), "password".to_string())
        .await;

    let response = test_app
        .publish_newsletter(
            Some(html),
            Some(text),
            Some(subject),
            "admin",
            Some("password"),
        )
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());

    let expected_emails = test_app.get_confirmed_subscriptions().await;
    assert_eq!(expected_emails, test_app.get_sent_emails().len());
}

#[tokio::test]
async fn newsletter_not_sent_to_unconfirmed_subscribers() {
    let test_app = spawn().await.unwrap();
    test_app
        .add_test_user("admin".to_string(), "password".to_string())
        .await;

    let name: String = FirstName().fake();
    let email: String = SafeEmail().fake();

    let response = test_app
        .create_subscription(name.clone(), email.clone())
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let response = test_app
        .publish_newsletter(
            Some(html),
            Some(text),
            Some(subject),
            "admin",
            Some("password"),
        )
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());

    let sent_emails = test_app.get_sent_emails();

    let unconfirmed_subscriber_emails: Vec<&(String, String, String)> = sent_emails
        .iter()
        .filter(|(recipient_email, _, _)| recipient_email == &email)
        .collect();

    assert_eq!(
        unconfirmed_subscriber_emails.len(),
        1,
        "Expected recipient to receive only confirmation email"
    );

    let (_, _, email_plaintext) = &unconfirmed_subscriber_emails[0];
    assert!(
        email_plaintext.contains("confirm your subscription"),
        "Expected confirmation email text"
    );
}
