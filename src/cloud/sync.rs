use std::collections::HashSet;
use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::{cloud::error::SyncError, data::Note, message::sync::NoteResponse};
use super::client::CloudClient;

impl CloudClient {
    pub async fn create_note(
        &self,
        id: &str,
        title: &str,
        created_at: DateTime<Utc>,
    ) -> Result<String, SyncError> {
        let response = self.client
            .post(&format!("{}/notes/{}", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .json(&serde_json::json!({
                "title": title,
                "created_at": created_at,
            }))
            .send()
            .await?;
        
        tracing::debug!("Create note response: {:?}", response);

        if response.status().is_success() {
            Ok(id.to_string())
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }

    pub async fn import_note(
        &self,
        id: &str,
        title: &str,
        content: &str,
        tags: &HashSet<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<String, SyncError> {
        let response = self.client
            .post(&format!("{}/notes/{}/import", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .json(&serde_json::json!({
                "title": title,
                "content": content,
                "tags": tags,
                "created_at": created_at,
                "updated_at": updated_at,
            }))
            .send()
            .await?;

        tracing::debug!("Import note response: {:?}", response);

        if response.status().is_success() {
            Ok(id.to_string())
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }

    pub async fn get_note(&self, id: &str) -> Result<Note, SyncError> {
        let response = self.client
            .get(&format!("{}/notes/{}", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        tracing::debug!("Get note response: {:?}", response);

        if response.status().is_success() {
            let json = response
                .json::<Value>().await
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            let note: Note = serde_json::from_value(json)
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            Ok(note)
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }

    pub async fn update_note(
        &self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
        tags: Option<HashSet<String>>,
        updated_at: DateTime<Utc>,
    ) -> Result<Note, SyncError> {
        let response = self.client
            .put(&format!("{}/notes/{}", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .json(&serde_json::json!({
                "title": title,
                "content": content,
                "tags": tags,
                "updated_at": updated_at,
            }))
            .send()
            .await?;

        tracing::debug!("Update note response: {:?}", response);

        if response.status().is_success() {
            let json = response
                .json::<Value>().await
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            let note: Note = serde_json::from_value(json)
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            Ok(note)
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }

    pub async fn delete_note(&self, id: &str) -> Result<String, SyncError> {
        let response = self.client
            .delete(&format!("{}/notes/{}", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        tracing::debug!("Delete note response: {:?}", response);

        if response.status().is_success() {
            Ok(id.to_string())
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }

    pub async fn sync_notes(
        &self,
        last_sync_time: Option<DateTime<Utc>>,
    ) -> Result<NoteResponse, SyncError> {
        let response = self.client
            .post(&format!("{}/notes/sync", self.base_url))
            .bearer_auth(&self.auth_token)
            .json(&serde_json::json!({
                "last_sync_time": last_sync_time,
                "device_id": "client_device"
            }))
            .send()
            .await?;

        tracing::debug!("Sync notes response: {:?}", response);

        if response.status().is_success() {
            let json = response
                .json::<Value>().await
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            let note_response: NoteResponse = serde_json::from_value(json)
                .map_err(|e| SyncError::JsonSerdeError(e.to_string()))?;
            Ok(note_response)
        } else {
            Err(SyncError::ServerError("Unknown error".into()))
        }
    }
}