//! tests/api/healh_check.rs

use crate::utils::spawn_app;

#[tokio::test]
async fn health_check_ok() {
    let app_address = spawn_app().await.unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
