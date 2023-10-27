pub mod client;
pub mod request;
pub mod response;
pub mod token;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Token io error: {0}")]
    TokenIo(#[from] anyhow::Error),

    #[error("Client field not defined: {field}")]
    ClientFieldUndefined { field: String },

    #[error("TransferRequest field not defined: {field}")]
    TransferRequestFieldUndefined { field: String },

    #[error("Payment field not defined: {field}")]
    PaymentFieldUndefined { field: String },

    #[error("Access Token not set")]
    NoAccessToken,

    #[error("Request error: {0}")]
    CustomRequest(String),
}

#[cfg(test)]
mod tests;
