use crate::test_app::spawn;
use fake::{faker, uuid::UUIDv4, Fake};

#[tokio::test]
async fn confirm_returns_200_with_valid_token() {
    let test_app = spawn().await.unwrap();

    let name: String = faker::name::en::FirstName().fake();
    let email: String = faker::internet::en::SafeEmail().fake();

    let response = test_app
        .create_subscription(name.clone(), email.clone())
        .await
        .expect("Failed to post subscription");
    assert_eq!(200, response.status().as_u16());

    let subscriber_id = test_app.get_subscription(&name, &email).await;
    let subscription_token = test_app.get_subscription_token(subscriber_id).await;

    let response = test_app
        .confirm_subscription(&subscription_token)
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn confirm_returns_400_with_invalid_token() {
    let test_app = spawn().await.unwrap();

    let uuid: String = UUIDv4.fake();

    let response = test_app
        .confirm_subscription(&uuid)
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn confirm_returns_400_with_no_token() {
    let test_app = spawn().await.unwrap();

    let response = test_app
        .confirm_subscription_no_token()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}
