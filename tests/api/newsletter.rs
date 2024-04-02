//! tests/api/newsletter.rs

use crate::test_app::spawn;

#[tokio::test]
async fn publish_newsletter_returns_200() {
    let test_app = spawn().await.unwrap();

    let response = test_app
        .publish_newsletter()
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());
}
