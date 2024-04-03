//! tests/api/newsletter.rs

use crate::test_app::spawn;
use fake::Fake;
use fake::faker::lorem::en::Paragraph;

#[tokio::test]
async fn publish_newsletter_returns_200() {
    let paragraph: String = Paragraph(1..2).fake();

    let test_app = spawn().await.unwrap();

    let response = test_app
        .publish_newsletter(paragraph)
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());
}
