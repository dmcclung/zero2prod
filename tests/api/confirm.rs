use crate::test_app::spawn;

#[tokio::test]
async fn confirm_returns_200_with_valid_token() {
    let test_app = spawn().await.unwrap();

    let response = test_app
        .confirm_subscription()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}
