//! tests/api/healh_check.rs

use crate::test_app::spawn;

#[tokio::test]
async fn health_check_ok() {
    let test_app = spawn().await.unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", test_app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
