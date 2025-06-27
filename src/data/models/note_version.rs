use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use super::note::Note;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteVersion {
    pub(in crate::data) id: String,             // UUID
    pub(in crate::data) note_id: String,
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) tags: HashSet<String>,
    pub(in crate::data) comment: String,
    pub(in crate::data) saved_at: DateTime<Utc>,
}

impl NoteVersion {
    pub fn new(comment: &str, note: &Note) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            note_id: note.id().to_string(),
            title: note.title().to_string(),
            content: note.content().to_string(),
            tags: note.tags().clone(),
            comment: comment.to_string(),
            saved_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn note_id(&self) -> &str {
        &self.note_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn saved_at(&self) -> String {
        format!("{}", self.saved_at.with_timezone(&Local).format("%Y-%m-%d %H:%M"))
    }
}
