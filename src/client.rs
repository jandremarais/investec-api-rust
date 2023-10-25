use std::collections::HashMap;

use crate::{
    token::{AccessToken, AccessTokenResponse, FileStore, TokenStore},
    Error,
};

pub struct Client<T: TokenStore> {
    id: String,
    secret: String,
    key: String,
    host: Host,
    pub access_token: Option<AccessToken>,
    token_store: Option<T>,
    http_client: reqwest::Client,
}

impl Client<FileStore> {
    /// Create a client to the Investec Sandbox environment with a local token store
    pub fn sandbox() -> Self {
        Self {
            id: "yAxzQRFX97vOcyQAwluEU6H6ePxMA5eY".to_string(),
            secret: "4dY0PjEYqoBrZ99r".to_string(),
            key: "eUF4elFSRlg5N3ZPY3lRQXdsdUVVNkg2ZVB4TUE1ZVk6YVc1MlpYTjBaV010ZW1FdGNHSXRZV05qYjNWdWRITXRjMkZ1WkdKdmVBPT0=".to_string(),
            host: Host::Sandbox,
            access_token: None,
            token_store: Some(FileStore::default()),
            http_client: reqwest::Client::new(),
        }
    }
}
impl<T: TokenStore> Client<T> {
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
