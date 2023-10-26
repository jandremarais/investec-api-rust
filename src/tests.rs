use chrono::NaiveDate;

use crate::client::{
    Beneficiary, Client, MultiTransferRequest, TransactionType, TransferBuilder, TransferRequest,
};

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

const SANDBOX_ACCOUNT: &'static str = "3353431574710163189587446";
const SANDBOX_PROFILE: &'static str = "10163189587444";

#[tokio::test]
async fn test_get_accounts() {
    let mut client = Client::sandbox();
    let accounts = client.get_accounts().await;
    assert!(accounts.is_ok());
}

#[tokio::test]
async fn test_get_account_balance() {
    let mut client = Client::sandbox();
    let balance = client.get_account_balance(SANDBOX_ACCOUNT).await;
    assert!(balance.is_ok());
}

#[tokio::test]
async fn test_get_account_transactions() {
    let mut client = Client::sandbox();

    let from_date = NaiveDate::from_ymd_opt(2023, 10, 1);
    let to_date = NaiveDate::from_ymd_opt(2023, 10, 3);
    let t_type = TransactionType::CardPurchases;
    let transactions = client
        .get_account_transactions(SANDBOX_ACCOUNT, from_date, to_date, Some(t_type))
        .await;
    assert!(transactions.is_ok());
    let transactions = transactions.unwrap();
    assert!(transactions.data.transactions.len() > 0);

    for t in transactions.data.transactions {
        assert_eq!(t.transaction_type, t_type);
        assert!(t.transaction_date <= to_date.unwrap());
        assert!(t.transaction_date >= from_date.unwrap());
    }
}

#[tokio::test]
async fn test_get_profiles() {
    let mut client = Client::sandbox();
    let profiles = client.get_profiles().await;
    assert!(profiles.is_ok());
}

#[tokio::test]
async fn test_get_profile_accounts() {
    let mut client = Client::sandbox();
    let resp = client.get_profile_accounts(SANDBOX_PROFILE).await;
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert!(resp.data.len() > 0);
    for a in resp.data {
        assert_eq!(a.profile_id, SANDBOX_PROFILE)
    }
}

#[tokio::test]
async fn test_get_auth_setup_details() {
    let mut client = Client::sandbox();
    let resp = client
        .get_auth_setup_details(SANDBOX_PROFILE, SANDBOX_ACCOUNT)
        .await;
    assert!(resp.is_ok());
}

// this is failing!
#[tokio::test]
async fn test_get_profile_beneficiaries() {
    let mut client = Client::sandbox();

    // let p_accounts = client.get_profile_accounts("10182035050680").await.unwrap();
    // dbg!(p_accounts);
    let resp = client
        .get_profile_beneficiaries(SANDBOX_PROFILE, SANDBOX_ACCOUNT)
        // .get_profile_beneficiaries("10182035050680", "3882638263210182035137740")
        .await;
    // match resp {
    //     Ok(r) => {}
    //     Err(e) => println!("{e}"),
    // }
    assert!(resp.is_ok());
}

#[tokio::test]
async fn test_get_beneficiaries() {
    let mut client = Client::sandbox();
    let resp = client.get_beneficiaries().await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn test_transfer_multiple() {
    let mut client = Client::sandbox();
    let transfer1 = TransferBuilder::new()
        .beneficiary_account_id(SANDBOX_ACCOUNT)
        .amount(10.0)
        .my_reference("test")
        .their_reference("test theirs")
        .build()
        .unwrap();
    let transfer2 = TransferBuilder::new()
        .beneficiary_account_id(SANDBOX_ACCOUNT)
        .amount(1.0)
        .my_reference("test2")
        .their_reference("test2 theirs")
        .build()
        .unwrap();
    let req = MultiTransferRequest::new(vec![transfer1, transfer2], None);
    let resp = client.transfer_multiple(SANDBOX_ACCOUNT, req).await;
    // if let Err(e) = resp {
    //     dbg!(e);
    // }
    assert!(resp.is_ok());
}

#[tokio::test]
async fn test_transfer_single() {
    // wait 2 seconds to avoid hitting burst limits
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let mut client = Client::sandbox();
    let transfer = TransferBuilder::new()
        .beneficiary_account_id(SANDBOX_ACCOUNT)
        .amount(10.0)
        .my_reference("test")
        .their_reference("test theirs")
        .build()
        .unwrap();
    let resp = client
        .transfer_single(SANDBOX_ACCOUNT, transfer, None)
        .await;
    // if let Err(e) = resp {
    //     dbg!(e);
    // }
    assert!(resp.is_ok());
}
