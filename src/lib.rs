use std::{collections::HashMap, str::FromStr};

use reqwest::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),
}

pub struct Client {
    id: String,
    secret: String,
    key: String,
    host: Host,
    access_token: Option<AccessToken>,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a client to the Investec Sandbox environment
    pub fn sandbox() -> Self {
        Self {
            id: "yAxzQRFX97vOcyQAwluEU6H6ePxMA5eY".to_string(),
            secret: "4dY0PjEYqoBrZ99r".to_string(),
            key: "eUF4elFSRlg5N3ZPY3lRQXdsdUVVNkg2ZVB4TUE1ZVk6YVc1MlpYTjBaV010ZW1FdGNHSXRZV05qYjNWdWRITXRjMkZ1WkdKdmVBPT0=".to_string(),
            host: Host::Sandbox,
            access_token: None,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_access_token(&self) -> Result<AccessToken, Error> {
        let url = format!("{}/identity/v2/oauth2/token", self.host.url());
        let params: HashMap<&str, &str> =
            HashMap::from_iter([("grant_type", "client_credentials")]);
        let resp = self
            .http_client
            .post(url)
            .basic_auth(&self.id, Some(&self.secret))
            .header("x-api-key", &self.key)
            .form(&params)
            .send()
            .await?;
        let resp = resp.error_for_status()?;
        let token: AccessToken = resp.json().await?;
        dbg!(&token);
        Ok(token)
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
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: u32,
    #[serde(deserialize_with = "from_space_separated")]
    scope: Vec<Scope>,
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

#[derive(Debug, Deserialize, Serialize)]
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
