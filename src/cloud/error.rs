use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Url parse error: {0}")]
    UrlParseError(String),

    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("Json serde error: {0}")]
    JsonSerdeError(String),

    #[error("Response error: {0}")]
    ResponseError(String),
}

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("Json serde error: {0}")]
    JsonSerdeError(String),

    #[error("Server error: {0}")]
    ServerError(String),
}