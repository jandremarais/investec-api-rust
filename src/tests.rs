use super::*;

#[tokio::test]
async fn test_get_access_token() {
    let mut client = Client::sandbox();

    let token = client.get_access_token().await;

    assert!(token.is_ok());
}
