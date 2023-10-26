pub mod client;
pub mod token;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Token io error: {0}")]
    TokenIo(#[from] anyhow::Error),

    #[error("Client field not defined: {field}")]
    ClientFieldUndefined { field: String },

    #[error("Access Token not set")]
    NoAccessToken,
}

#[cfg(test)]
mod tests;
