use std::{collections::HashMap, path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Token io error: {0}")]
    TokenIo(#[from] anyhow::Error),
}

pub struct Client<T: TokenStore> {
    id: String,
    secret: String,
    key: String,
    host: Host,
    access_token: Option<AccessToken>,
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

pub trait TokenStore {
    fn read(&self) -> anyhow::Result<AccessToken>;
    fn write(&self, token: &AccessToken) -> anyhow::Result<()>;
}

pub struct FileStore {
    pub path: PathBuf,
}

impl FileStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Default for FileStore {
    fn default() -> Self {
        Self::new(PathBuf::from("token.json"))
    }
}

impl TokenStore for FileStore {
    fn read(&self) -> anyhow::Result<AccessToken> {
        let body = std::fs::read_to_string(&self.path)?;
        let token = serde_json::from_str(&body)?;
        Ok(token)
    }

    fn write(&self, token: &AccessToken) -> anyhow::Result<()> {
        let body = serde_json::to_string_pretty(token)?;
        std::fs::write(&self.path, body)?;
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

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    #[serde(deserialize_with = "from_space_separated")]
    scope: Vec<Scope>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    scope: Vec<Scope>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl AccessToken {
    fn expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }
}

impl From<AccessTokenResponse> for AccessToken {
    fn from(value: AccessTokenResponse) -> Self {
        Self {
            access_token: value.access_token,
            token_type: value.token_type,
            scope: value.scope,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(value.expires_in as i64),
        }
    }
}

fn from_space_separated<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: Deserializer<'de>,
{
    let body: String = Deserialize::deserialize(deserializer)?;
    let array = body
        .split_whitespace()
        .map(|o| Scope::from_str(o).unwrap())
        .collect();
    Ok(array)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Scope {
    Accounts,
    Balances,
    Transactions,
    Transfers,
    Beneficiarypayments,
    DocumentsStatements,
    DocumentsTaxcertificates,
}
impl FromStr for Scope {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accounts" => Ok(Self::Accounts),
            "balances" => Ok(Self::Balances),
            "transactions" => Ok(Self::Transactions),
            "transfers" => Ok(Self::Transfers),
            "beneficiarypayments" => Ok(Self::Beneficiarypayments),
            "documents.statements" => Ok(Self::DocumentsStatements),
            "documents.taxcertificates" => Ok(Self::DocumentsTaxcertificates),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests;

// pub struct ClientBuilder {
//     id: Option<String>,
//     secret: Option<String>,
//     host: Option<Host>,
//     access_token: Option<AccessToken>,
// }
// impl ClientBuilder {
//     pub fn new() -> Self {
//         Self {
//             id: None,
//             secret: None,
//             host: Some(Host::Live),
//             access_token: None,
//         }
//     }

//     fn id(mut self, id: String) -> Self {
//         self.id = Some(id);
//         self
//     }
// }
