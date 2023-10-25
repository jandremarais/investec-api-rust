use super::*;

#[tokio::test]
async fn test_get_access_token() {
    let client = Client::sandbox();
    let token = client.get_access_token().await;
    assert!(token.is_ok());
}

#[tokio::test]
async fn test_authenticate() {
    let mut client = Client::sandbox();

    client.authenticate().await.unwrap();
    let token1 = client.access_token.clone().unwrap();

    assert!(!token1.expired());

    // is it using the same token stored in the client
    client.authenticate().await.unwrap();
    let token2 = client.access_token.unwrap();
    assert_eq!(token1.access_token, token2.access_token);

    // is it using the same token stored in the file store
    let mut client = Client::sandbox();
    client.authenticate().await.unwrap();
    let mut token3 = client.access_token.clone().unwrap();
    assert_eq!(token1.access_token, token3.access_token);

    // is it getting a new token if expired
    token3.expires_at = chrono::Utc::now();
    assert!(token3.expired());
    client.access_token = Some(token3);
    client.authenticate().await.unwrap();
    let token4 = client.access_token.clone().unwrap();
    assert_ne!(token1.access_token, token4.access_token);
}
