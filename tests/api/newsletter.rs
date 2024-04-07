//! tests/api/newsletter.rs

use crate::test_app::spawn;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::Fake;

#[tokio::test]
async fn publish_newsletter_returns_200() {
    let text: String = Paragraph(1..2).fake();
    let html = String::from(format!("<p>{}</p>", text));
    let subject = Sentence(1..2).fake();

    let test_app = spawn().await.unwrap();

    let response = test_app
        .publish_newsletter(html, text, subject)
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());

    let expected_emails = test_app.get_confirmed_subscriptions().await;
    assert_eq!(expected_emails, test_app.get_sent_emails().len());
}
