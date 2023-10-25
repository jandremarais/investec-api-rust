use std::collections::HashMap;

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

pub struct ClientBuilder {
    id: Option<String>,
    secret: Option<String>,
    key: Option<String>,
    host: Option<Host>,
    token_store: Option<Box<dyn TokenStore>>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            secret: None,
            key: None,
            host: None,
            token_store: None,
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

        let host = self.host.unwrap_or(Host::Live);
        let client = Client {
            id,
            secret,
            key,
            host,
            access_token: None,
            token_store: self.token_store,
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
}
