use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

// Token as received from API
#[derive(Deserialize, Serialize, Debug)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    #[serde(deserialize_with = "from_space_separated")]
    scope: Vec<Scope>,
}

// Token used by the client
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: Vec<Scope>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl AccessToken {
    pub fn expired(&self) -> bool {
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

// to deserialize "accounts balances ..." -> Vec<Scope>
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
pub enum Scope {
    Accounts,
    Balances,
    Transactions,
    Transfers,
    Beneficiarypayments,
    Statements,
    Taxcertificates,
    Cards,
}

// TODO: is there a better way to deserialize this enum?
impl FromStr for Scope {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accounts" => Ok(Self::Accounts),
            "balances" => Ok(Self::Balances),
            "transactions" => Ok(Self::Transactions),
            "transfers" => Ok(Self::Transfers),
            "beneficiarypayments" => Ok(Self::Beneficiarypayments),
            "statements" => Ok(Self::Statements),
            "taxcertificates" => Ok(Self::Taxcertificates),
            "cards" => Ok(Self::Cards),
            _ => Err(()),
        }
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
