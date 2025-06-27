use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use super::note::Note;

// 导出笔记数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportNote {
    pub(crate) id: Option<String>,
    pub(crate) title: String,
    tags: HashSet<String>,
    content: String,
    pub(crate) created: Option<DateTime<Utc>>,
    pub(crate) updated: Option<DateTime<Utc>>,
}

impl ExportNote {
    pub fn new(
        id: Option<String>, 
        title: String, 
        tags: HashSet<String>, 
        content: String, 
        created: Option<DateTime<Utc>>,
        updated: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            tags,
            content,
            created,
            updated,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created(&self) -> Option<String> {
        self.created.and_then(|time| 
            Some(format!("{}", time.with_timezone(&Local).format("%Y-%m-%d %H:%M")))
        )
    }

    pub fn to_note(&self) -> Note {
        Note {
            id: self.id.clone().unwrap_or(uuid::Uuid::new_v4().to_string()),
            title: self.title.clone(),
            content: self.content.clone(),
            tags: self.tags.clone(),
            created_at: self.created.unwrap_or(Utc::now()),
            updated_at: self.updated.unwrap_or(Utc::now()),
            is_pinned: false,
        }
    }

    pub fn from_note(note: &Note) -> Self {
        Self {
            id: Some(note.id.clone()),
            title: note.title.clone(),
            content: note.content.clone(),
            tags: note.tags.clone(),
            created: Some(note.created_at),
            updated: Some(note.updated_at)
        }
    }
}