pub mod client;
pub mod token;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Token io error: {0}")]
    TokenIo(#[from] anyhow::Error),
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
