use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{cloud::error::SyncError, data::Note};


#[derive(Debug)]
pub enum SyncMessage{
    CreateNote {
        id: String,
        title: String,
        created_at: DateTime<Utc>,
    },
    UpdateNote {
        id: String,
        title: Option<String>,
        content: Option<String>,
        tags: Option<HashSet<String>>,
        updated_at: DateTime<Utc>,
    },
    ImportNote {
        id: String,
        title: String,
        content: String,
        tags: HashSet<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    },
    DeleteNote {
        id: String
    },
    SyncNotes {
        last_sync_time: Option<DateTime<Utc>>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    success: bool,
    sync_data: Option<SyncData>,
    error: Option<String>,
}

impl From<Result<String, SyncError>> for SyncResponse {
    fn from(result: Result<String, SyncError>) -> Self {
        match result {
            Ok(id) => Self { success: true, sync_data: Some(SyncData::NoteId(id)), error: None },
            Err(e) => Self { success: false, sync_data: None, error: Some(e.to_string()) }
        }
    }
}

impl From<Result<Note, SyncError>> for SyncResponse {
    fn from(result: Result<Note, SyncError>) -> Self {
        match result {
            Ok(note) => Self { success: true, sync_data: Some(SyncData::Note(note)), error: None },
            Err(e) => Self { success: false, sync_data: None, error: Some(e.to_string()) }
        }
    }
}

impl From<Result<NoteResponse, SyncError>> for SyncResponse {
    fn from(result: Result<NoteResponse, SyncError>) -> Self {
        match result {
            Ok(note_resp) => Self { success: true, sync_data: Some(SyncData::NoteResponse(note_resp)), error: None },
            Err(e) => Self { success: false, sync_data: None, error: Some(e.to_string()) }
        }
    }
}

impl SyncResponse {
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn sync_data(&self) -> Option<SyncData> {
        self.sync_data.clone()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteResponse {
    pub notes: Vec<Note>,
    pub deleted_note_ids: Vec<String>,
    pub current_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum SyncData {
    Note(Note),
    NoteId(String),
    NoteResponse(NoteResponse),
}