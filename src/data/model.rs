use super::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub(crate) id: String,             // UUID
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) tags: Vec<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) is_pinned: bool,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
        }
    }

    pub fn contains(&self, key: &String, case_sensitive: Option<bool>) -> bool {
        let case_sensitive = case_sensitive.unwrap_or(false);
        if case_sensitive {
            self.title.contains(key) || self.content.contains(key)
        } else {
            self.title.to_lowercase().contains(&key.to_lowercase())
            || self.content.to_lowercase().contains(&key.to_lowercase())
        }
    }
}

#[derive(Debug, Default)]
pub struct Notebook {
    pub(crate) notes: HashMap<String, Note>,
    pub(crate) _tags: HashMap<String, i32>,
}

impl Notebook {
    pub fn add_note(&mut self, note: Note) {
        self.notes.insert(note.id.clone(), note);
    }

    pub fn delete_note(&mut self, note_id: &String) {
        self.notes.remove(note_id);
    }
}