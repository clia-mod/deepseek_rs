use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientInitErrors {
    #[error("Error getting API key from environment variable")]
    DeepSeekApiKeyNotSet(#[from] std::env::VarError),
    #[error("Unknown error")]
    Unknown,
}
