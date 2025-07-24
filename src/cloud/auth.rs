use serde_json::{json, Value};

use super::{client::CloudClient, error::AuthError};

impl CloudClient {
    // 注册
    pub async fn register(&mut self, name: &str, email: &str, password: &str) -> Result<Option<(String, String)>, AuthError> {
        let register_res = self.client
            .post(&format!("{}/auth/register", self.base_url))
            .json(&json!({
                "name": name,
                "email": email,
                "password": password
            }))
            .send()
            .await?;

        tracing::debug!("Register response: {:?}", register_res);
        
        match register_res.error_for_status() {
            Ok(response) => {
                let auth_values = response
                    .json::<Value>().await
                    .map_err(|e| AuthError::JsonSerdeError(e.to_string()))?;
                self.auth_token = auth_values["token"].to_string();
                let user_id = auth_values["user_id"].to_string().trim_matches('"').to_string();
                let user_name = auth_values["user_name"].to_string().trim_matches('"').to_string();
                
                tracing::info!("Register successfully. User id: {}", user_id);
                Ok(Some((user_name, user_id)))
            }
            Err(err) => {
                Err(AuthError::ResponseError(err.to_string()))
            }
        }
    }

    // 登录
    pub async fn login(&mut self, email: &str, password: &str) -> Result<Option<(String, String)>, AuthError> {
        let login_res = self.client
            .get(&format!("{}/auth/login", self.base_url))
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await?;
        
        tracing::debug!("Login response: {:?}", login_res);

        match login_res.error_for_status() {
            Ok(response) => {
                let auth_values = response
                    .json::<Value>().await
                    .map_err(|e| AuthError::JsonSerdeError(e.to_string()))?;
                self.auth_token = auth_values["token"].to_string();
                let user_id = auth_values["user_id"].to_string().trim_matches('"').to_string();
                let user_name = auth_values["user_name"].to_string().trim_matches('"').to_string();
                
                tracing::info!("Login successfully. User name: {}", user_name);
                Ok(Some((user_name, user_id)))
            }
            Err(err) => {
                Err(AuthError::ResponseError(err.to_string()))
            }
        }
    }

    // 访问受保护资源
    pub async fn get_me(&self) -> Result<(), AuthError> {
        let protected_res = self.client
            .get(&format!("{}/me", self.base_url))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        tracing::debug!("Protected response: {:?}", protected_res);

        Ok(())
    }

    // 登出
    pub async fn logout(&self) -> Result<(), AuthError> {
        let logout_res = self.client
            .post(&format!("{}/logout", self.base_url))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        tracing::debug!("Logout response: {:?}", logout_res);
        
        Ok(())
    }
}