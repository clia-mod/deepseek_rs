use reqwest::{Error as ReqwestError, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestErrors {
    #[error("HTTP Error: {0}")]
    HttpError(ReqwestError), // Removed #[from]

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("TLS error: {0}")]
    TlsError(String),

    #[error("Redirect error: {0}")]
    RedirectError(String),

    #[error("Body error: {0}")]
    BodyError(String),

    #[error("Builder error: {0}")]
    BuilderError(String),

    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Status {0}: {1}")]
    StatusError(StatusCode, String),
    #[error("Unknown error")]
    Unknown,
}

impl From<ReqwestError> for RequestErrors {
    fn from(error: ReqwestError) -> Self {
        if let Some(status) = error.status() {
            match status {
                StatusCode::BAD_REQUEST => RequestErrors::BadRequest(error.to_string()),
                StatusCode::UNAUTHORIZED => RequestErrors::Unauthorized(error.to_string()),
                StatusCode::FORBIDDEN => RequestErrors::Forbidden,
                StatusCode::TOO_MANY_REQUESTS => {
                    RequestErrors::RateLimitExceeded(error.to_string())
                }
                _ => RequestErrors::StatusError(status, error.to_string()),
            }
        } else if error.is_timeout() {
            RequestErrors::TimeoutError(error.to_string())
        } else if error.is_connect() {
            RequestErrors::ConnectionError(error.to_string())
        } else if error.is_redirect() {
            RequestErrors::RedirectError(error.to_string())
        } else if error.is_body() {
            RequestErrors::BodyError(error.to_string())
        } else if error.is_builder() {
            RequestErrors::BuilderError(error.to_string())
        } else if error.is_decode() {
            RequestErrors::DecodeError(error.to_string())
        } else {
            RequestErrors::HttpError(error)
        }
    }
}
