use serde::{Serialize, Deserialize};

use crate::cloud::error::AuthError;

#[derive(Debug)]
pub enum AuthMessage {
    Login {
        email: String,
        password: String,
    },
    Register {
        name: String,
        email: String,
        password: String,
    },
    Logout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    success: bool,
    user_info: Option<(String, String)>,    // (name, id)
    error: Option<String>,
}

impl From<Result<Option<(String, String)>, AuthError>> for AuthResponse {
    fn from(result: Result<Option<(String, String)>, AuthError>) -> Self {
        match result {
            Ok(user_info) => Self { success: true, user_info, error: None },
            Err(e) => Self { success: false, user_info: None, error: Some(e.to_string()) }
        }
    }
}

impl From<Result<(), AuthError>> for AuthResponse {
    fn from(result: Result<(), AuthError>) -> Self {
        match result {
            Ok(_) => Self { success: true, user_info: None, error: None },
            Err(e) => Self { success: false, user_info: None, error: Some(e.to_string()) }
        }
    }
}

impl AuthResponse {
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn user_name(&self) -> Option<String> {
        self.user_info.as_ref().map(|(name, _)| name.clone())
    }

    pub fn user_id(&self) -> &str {
        self.user_info.as_ref().map_or("", |(_, id)| id.as_str())
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}