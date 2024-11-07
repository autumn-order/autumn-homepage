use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReqwestOrHttpError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("HTTP error: {0}")]
    Http(HttpError),
}

#[derive(Deserialize, Debug)]
pub struct HttpError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP error {}: {}", self.code, self.message)
    }
}
