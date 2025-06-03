use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::db::{DbConnection, DbResult};

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
    pub fn load_from_db(conn: &DbConnection) -> DbResult<Self> {
        let mut stmt = conn.prepare(
            "SELECT id, title, content, tags, create_at, update_at, is_pinned FROM notes"
        )?;
        let note_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap().with_timezone(&Utc),
                is_pinned: row.get(6)?,
            })
        })?;

        let mut notebook = Notebook::default();
        for note in note_iter {
            let note = note?;
            notebook.add_note(note);
        }
        Ok(notebook)
    }

    pub fn save_to_db(&self, conn: &mut DbConnection) -> DbResult<()> {
        let tx = conn.transaction()?;

        for note in self.notes.values() {
            tx.execute(
                "", 
                rusqlite::params![
                    note.id,
                    note.title,
                    note.content,
                    serde_json::to_string(&note.tags).unwrap(),
                    note.created_at.to_rfc3339(),
                    note.updated_at.to_rfc3339(),
                    note.is_pinned as i32,
                ]
            )?;
        }

        tx.commit()
    }

    pub fn add_note(&mut self, note: Note) {
        self.tags.extend(note.tags.clone());
        self.notes.insert(note.id.clone(), note);
    }
}