use reqwest::{Client, Url};
// use serde::{Serialize, de::DeserializeOwned};
use super::error::AuthError;

pub struct CloudClient {
    pub(super) client: Client,
    pub(super) base_url: Url,

    pub(super) auth_token: String,
}

impl CloudClient {
    pub fn new(base_url: &str) -> Result<Self, AuthError> {
        let base_url = Url::parse(base_url)
            .map_err(|e| AuthError::UrlParseError(e.to_string()))?;
        Ok(Self {
            client: Client::new(),
            base_url,
            auth_token: String::new(),
        })
    }
}