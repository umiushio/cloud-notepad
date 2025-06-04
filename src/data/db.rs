use rusqlite::{Connection, Result, Transaction};

use super::*;
use crate::{Note, Notebook};

pub type DbResult<T> = Result<T>;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> DbResult<Self> {
        let connection = Connection::open("notes.db")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_pinned INTEGER DEFAULT 0
            )",
            [],
        )?;
        Ok( Self { connection } )
    } 

    pub fn load_all_notes(&self) -> DbResult<Notebook> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, tags, created_at, updated_at, is_pinned FROM notes"
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

    fn insert_or_replace_note(
        tx: &Transaction,
        note: &Note,
    ) -> DbResult<()> {
        tx.execute(
            "INSERT OR REPLACE INTO notes
            (id, title, content, tags, created_at, updated_at, is_pinned)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
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
        Ok(())
    }

    pub fn save_note(&mut self, note: &Note) -> DbResult<()> {
        let tx = self.connection.transaction()?;
        Self::insert_or_replace_note(&tx, note)?;
        tx.commit()
    }

    pub fn save_notebook(&mut self, notebook: &Notebook) -> DbResult<()> {
        let tx = self.connection.transaction()?;
        for note in notebook.notes.values() {
            Self::insert_or_replace_note(&tx, note)?;
        }
        tx.commit()
    }   
}
