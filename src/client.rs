use std::collections::HashMap;

use chrono::NaiveDate;
use reqwest::Method;

use crate::{
    request::{MultiTransferRequest, MutliPaymentRequest, Payment, Transfer},
    response::{
        Account, AccountBalance, Accounts, Beneficiary, BeneficiaryCategory, MultiPaymentResponse,
        MultiTransferResponse, Profile, Response, SinglePaymentResponse, SingleTransferResponse,
        TransactionType, Transactions,
    },
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

    /// helper function to reduce repetitve code for autorefresh and http client setup
    // async fn default_get(&mut self, url: String) -> Result<reqwest::RequestBuilder, Error> {
    async fn default_request(
        &mut self,
        method: Method,
        url: String,
    ) -> Result<reqwest::RequestBuilder, Error> {
        if self.refresh_auth {
            self.authenticate().await?;
        }
        match &self.access_token {
            Some(token) => {
                let resp = self
                    .http_client
                    .request(method, url)
                    .bearer_auth(&token.access_token);
                // let resp = self.http_client.get(url).bearer_auth(&token.access_token);
                Ok(resp)
            }
            None => Err(Error::NoAccessToken),
        }
    }

    pub async fn get_accounts(&mut self) -> Result<Response<Accounts>, Error> {
        let url = format!("{}/za/pb/v1/accounts", self.host.url());
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;
        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn get_account_balance(
        &mut self,
        account_id: impl Into<String>,
    ) -> Result<Response<AccountBalance>, Error> {
        let url = format!(
            "{}/za/pb/v1/accounts/{}/balance",
            self.host.url(),
            account_id.into()
        );

        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;
        let data = resp.json().await?;

        Ok(data)
    }

    pub async fn get_account_transactions(
        &mut self,
        account_id: impl Into<String>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        transaction_type: Option<TransactionType>,
    ) -> Result<Response<Transactions>, Error> {
        let url = format!(
            "{}/za/pb/v1/accounts/{}/transactions",
            self.host.url(),
            account_id.into()
        );
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .query(&[("toDate", to_date), ("fromDate", from_date)])
            .query(&[("transactionType", transaction_type)])
            .send()
            .await?
            .error_for_status()?;
        let data = resp.json().await?;

        Ok(data)
    }

    pub async fn get_profiles(&mut self) -> Result<Response<Vec<Profile>>, Error> {
        let url = format!("{}/za/pb/v1/profiles", self.host.url(),);
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;

        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn get_profile_accounts(
        &mut self,
        profile_id: impl Into<String>,
    ) -> Result<Response<Vec<Account>>, Error> {
        let url = format!(
            "{}/za/pb/v1/profiles/{}/accounts",
            self.host.url(),
            profile_id.into()
        );
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;

        let data = resp.json().await?;
        Ok(data)
    }

    // TODO!: define struct for response data
    // not sure what all the possiblities are yet
    pub async fn get_auth_setup_details(
        &mut self,
        profile_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let url = format!(
            "{}/za/pb/v1/profiles/{}/accounts/{}/authorisationsetupdetails",
            self.host.url(),
            profile_id.into(),
            account_id.into()
        );
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;

        let data = resp.json().await?;
        Ok(data)
    }

    // TODO!: figure out why this is returning 404
    pub async fn get_profile_beneficiaries(
        &mut self,
        profile_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Response<Vec<Beneficiary>>, Error> {
        let url = format!(
            "{}/za/pb/v1/profiles/{}/beneficiaries/{}",
            self.host.url(),
            profile_id.into(),
            account_id.into()
        );

        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;

        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn get_beneficiaries(&mut self) -> Result<Response<Vec<Beneficiary>>, Error> {
        let url = format!("{}/za/pb/v1/accounts/beneficiaries", self.host.url(),);
        let resp = self
            .default_request(Method::GET, url)
            .await?
            .send()
            .await?
            .error_for_status()?;

        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn transfer_multiple(
        &mut self,
        account_id: impl Into<String>,
        transfer_list: MultiTransferRequest,
    ) -> Result<Response<MultiTransferResponse>, Error> {
        let url = format!(
            "{}/za/pb/v1/accounts/{}/transfermultiple",
            self.host.url(),
            account_id.into()
        );
        let resp = self
            .default_request(Method::POST, url)
            .await?
            .json(&transfer_list)
            .send()
            .await?;
        let resp = error_for_status_with_text(resp).await?;

        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn transfer_single(
        &mut self,
        account_id: impl Into<String>,
        request: Transfer,
        profile_id: impl Into<Option<String>>,
    ) -> Result<SingleTransferResponse, Error> {
        let req = MultiTransferRequest::new(vec![request], profile_id.into());
        let multi = self.transfer_multiple(account_id, req).await?;
        let transfer_response = multi.data.transfer_responses.into_iter().next().unwrap();
        let data = SingleTransferResponse {
            error_message: multi.data.error_message,
            transfer_response,
        };
        Ok(data)
    }

    pub async fn get_beneficiary_categories(
        &mut self,
    ) -> Result<Response<Vec<BeneficiaryCategory>>, Error> {
        let url = format!(
            "{}/za/pb/v1/accounts/beneficiarycategories",
            self.host.url()
        );
        let resp = self.default_request(Method::GET, url).await?.send().await?;
        let resp = error_for_status_with_text(resp).await?;
        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn pay_multiple(
        &mut self,
        account_id: impl Into<String>,
        payment_list: MutliPaymentRequest,
    ) -> Result<Response<MultiPaymentResponse>, Error> {
        let url = format!(
            "{}/za/pb/v1/accounts/{}/paymultiple",
            self.host.url(),
            account_id.into()
        );
        let resp = self
            .default_request(Method::POST, url)
            .await?
            .json(&payment_list)
            .send()
            .await?;
        // TODO! handle specific api errors
        let resp = error_for_status_with_text(resp).await?;
        let data = resp.json().await?;
        Ok(data)
    }

    pub async fn pay_single(
        &mut self,
        account_id: impl Into<String>,
        payment: Payment,
    ) -> Result<SinglePaymentResponse, Error> {
        let multi = self
            .pay_multiple(account_id, MutliPaymentRequest::new(vec![payment]))
            .await?;
        let transfer_response = multi.data.transfer_responses.into_iter().next().unwrap();
        let data = SinglePaymentResponse {
            error_message: multi.data.error_message,
            transfer_response,
        };
        Ok(data)
    }
}

async fn error_for_status_with_text(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
    let status = resp.status();
    if status.is_client_error() || status.is_server_error() {
        let body = resp.text().await?;
        Err(Error::CustomRequest(body))
    } else {
        Ok(resp)
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
