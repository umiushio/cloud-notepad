use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,             // UUID
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_pinned: bool,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Notebook {
    pub notes: HashMap<String, Note>,
    pub tags: HashSet<String>,
}

impl Notebook {
    pub fn add_note(&mut self, note: Note) {
        self.tags.extend(note.tags.clone());
        self.notes.insert(note.id.clone(), note);
    }
}