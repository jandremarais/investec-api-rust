use std::{collections::HashMap, fmt::Display};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    token::{AccessToken, AccessTokenResponse, FileStore, TokenStore},
    Error,
};

pub struct Client {
    pub id: String,
    pub secret: String,
    pub key: String,
    pub host: Host,
    pub access_token: Option<AccessToken>,
    pub token_store: Option<Box<dyn TokenStore>>,
    pub refresh_auth: bool,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a client to the Investec Sandbox environment with a local token store
    pub fn sandbox() -> Self {
        Self {
            id: "yAxzQRFX97vOcyQAwluEU6H6ePxMA5eY".to_string(),
            secret: "4dY0PjEYqoBrZ99r".to_string(),
            key: "eUF4elFSRlg5N3ZPY3lRQXdsdUVVNkg2ZVB4TUE1ZVk6YVc1MlpYTjBaV010ZW1FdGNHSXRZV05qYjNWdWRITXRjMkZ1WkdKdmVBPT0=".to_string(),
            host: Host::Sandbox,
            access_token: None,
            token_store: Some(Box::new(FileStore::default())),
            refresh_auth: true,
            http_client: reqwest::Client::new(),
        }
    }
}
impl Client {
    /// Get access token
    pub async fn get_access_token(&self) -> Result<AccessTokenResponse, Error> {
        let url = format!("{}/identity/v2/oauth2/token", self.host.url());
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");
        let resp = self
            .http_client
            .post(url)
            .basic_auth(&self.id, Some(&self.secret))
            .header("x-api-key", &self.key)
            .form(&params)
            .send()
            .await?;
        let resp = resp.error_for_status()?;
        let token: AccessTokenResponse = resp.json().await?;
        Ok(token)
    }

    /// exchange client credentials for access token if tokens in caches don't exist
    /// or expired. Cache if new token is fetched.
    pub async fn authenticate(&mut self) -> Result<(), Error> {
        if let Some(token) = &self.access_token {
            if !token.expired() {
                return Ok(());
            }
        } else if let Some(token_store) = &self.token_store {
            if let Ok(token) = token_store.read() {
                if !token.expired() {
                    self.access_token = Some(token);
                    return Ok(());
                }
            }
        }

        let token = self.get_access_token().await?.into();
        if let Some(token_store) = &self.token_store {
            token_store.write(&token)?;
        }
        self.access_token = Some(token);

        Ok(())
    }

    pub async fn get_accounts(&mut self) -> Result<InvestecRespone<Accounts>, Error> {
        if self.refresh_auth {
            self.authenticate().await?;
        }

        let url = format!("{}/za/pb/v1/accounts", self.host.url());
        let token = &self.access_token.as_ref().unwrap().access_token;
        let resp = self.http_client.get(url).bearer_auth(token).send().await?;
        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn get_account_balnce(
        &mut self,
        account_id: &str,
    ) -> Result<InvestecRespone<AccountBalance>, Error> {
        if self.refresh_auth {
            self.authenticate().await?;
        }

        let url = format!(
            "{}/za/pb/v1/accounts/{}/balance",
            self.host.url(),
            account_id
        );
        let token = &self.access_token.as_ref().unwrap().access_token;

        let resp = self.http_client.get(url).bearer_auth(token).send().await?;
        let data = resp.json().await?;

        Ok(data)
    }

    pub async fn get_account_transactions(
        &mut self,
        account_id: &str,
        from_date: Option<chrono::NaiveDate>,
        to_date: Option<chrono::NaiveDate>,
        transaction_type: Option<TransactionType>,
    ) -> Result<InvestecRespone<Transactions>, Error> {
        if self.refresh_auth {
            self.authenticate().await?;
        }

        let url = format!(
            "{}/za/pb/v1/accounts/{}/transactions",
            self.host.url(),
            account_id
        );
        let token = &self.access_token.as_ref().unwrap().access_token;
        let resp = self
            .http_client
            .get(url)
            .bearer_auth(token)
            .query(&[("toDate", to_date), ("fromDate", from_date)])
            .query(&[("transactionType", transaction_type)])
            .send()
            .await?;
        let data = resp.json().await?;

        Ok(data)
    }
}

#[derive(Debug, Deserialize)]
pub struct InvestecRespone<T> {
    pub data: T,
    // TODO!: create struct
    pub links: Links,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub selfx: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DtCt {
    Debit,
    Credit,
}

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Posted,
    // is this all
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
// #[serde(rename_all = "PascalCase")]
pub enum TransactionType {
    VASTransactions,
    ATMWithdrawals,
    CardPurchases,
    FeesAndInterest,
    Deposits,
    OnlineBankingPayments,
    DebitOrders,
    FasterPay,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub account_id: String,
    #[serde(rename = "type")]
    pub typex: DtCt,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub description: String,
    pub card_number: String,
    pub posted_order: i32,
    pub posting_date: NaiveDate,
    pub value_date: NaiveDate,
    pub action_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub amount: f32,
    pub running_balance: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_id: String,
    pub current_balance: f32,
    pub available_balance: f32,
    pub budget_balance: Option<f32>,
    pub straight_balance: Option<f32>,
    pub cash_balance: Option<f32>,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_pages: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub account_number: String,
    pub account_name: String,
    pub reference_name: String,
    pub product_name: String,
    pub kyc_compliant: bool,
    pub profile_id: String,
    pub profile_name: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account ID: {}\nAccount Number: {}\nAccount Name: {}\nReference Name: {}\nProduct Name: {}\nKYC Compliant: {}\nProfile ID: {}\nProfile Name: {}",
            self.account_id,
            self.account_number,
            self.account_name,
            self.reference_name,
            self.product_name,
            self.kyc_compliant,
            self.profile_id,
            self.profile_name
        )
    }
}

impl Display for AccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account ID: {}\nCurrent Balance: {}\nAvailable Balance: {}\nBudget Balance: {}\nStraight Balance: {}\nCash Balance: {}\nCurrency: {}",
            self.account_id,
            self.current_balance,
            self.available_balance,
            // 0.0 probably confusing display
            self.budget_balance.unwrap_or(0.0),
            self.straight_balance.unwrap_or(0.0),
            self.cash_balance.unwrap_or(0.0),
            self.currency,
        )
    }
}

pub enum Host {
    Live,
    Sandbox,
}

impl Host {
    fn url(&self) -> String {
        match self {
            Self::Live => "https://openapi.investec.com".to_string(),
            Self::Sandbox => "https://openapisandbox.investec.com".to_string(),
        }
    }
}

pub struct ClientBuilder {
    id: Option<String>,
    secret: Option<String>,
    key: Option<String>,
    host: Option<Host>,
    token_store: Option<Box<dyn TokenStore>>,
    refresh_auth: Option<bool>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            secret: None,
            key: None,
            host: None,
            token_store: None,
            refresh_auth: None,
        }
    }

    /// read id, secret and key from environment varialbles
    pub fn from_env() -> Self {
        let id = Some(std::env::var("INVESTEC_CLIENT_ID").expect("INVESTEC_CLIENT_ID"));
        let secret = Some(std::env::var("INVESTEC_CLIENT_SECRET").expect("INVESTEC_CLIENT_SECRET"));
        let key = Some(std::env::var("INVESTEC_API_KEY").expect("INVESTEC_API_KEY"));

        Self {
            id,
            secret,
            key,
            host: Some(Host::Live),
            token_store: None,
            refresh_auth: None,
        }
    }

    pub fn build(self) -> Result<Client, Error> {
        let id = self.id.ok_or(Error::ClientFieldUndefined {
            field: "id".to_string(),
        })?;
        let secret = self.secret.ok_or(Error::ClientFieldUndefined {
            field: "secret".to_string(),
        })?;
        let key = self.key.ok_or(Error::ClientFieldUndefined {
            field: "key".to_string(),
        })?;

        let refresh_auth = *&self.refresh_auth.unwrap_or(false);

        let host = self.host.unwrap_or(Host::Live);
        let client = Client {
            id,
            secret,
            key,
            host,
            access_token: None,
            token_store: self.token_store,
            refresh_auth,
            http_client: reqwest::Client::new(),
        };
        Ok(client)
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn sandbox(mut self) -> Self {
        self.host = Some(Host::Sandbox);
        self
    }

    pub fn token_store<T: TokenStore + 'static>(mut self, store: T) -> Self {
        self.token_store = Some(Box::new(store));
        self
    }

    /// to set local file store for token
    pub fn local_token(mut self) -> Self {
        self.token_store = Some(Box::new(FileStore::default()));
        self
    }

    pub fn refresh_auth(mut self) -> Self {
        self.refresh_auth = Some(true);
        self
    }
}
