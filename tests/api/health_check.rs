//! tests/api/healh_check.rs

use claims::assert_ok;

use crate::utils::spawn_app;

#[tokio::test]
async fn health_check_ok() {
    let test_app = spawn_app().await.unwrap();

    assert_ok!(test_app.reset_subscriptions().await);

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", test_app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
