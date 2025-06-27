use rusqlite::{Connection, Result, Transaction};
use chrono::{DateTime, Utc};
use super::{Note, DeleteNote, NoteVersion, Notebook};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("notes.db")?;
        // 笔记信息表
        connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_deleted BOOLEAN DEFAULT FALSE,
                is_pinned BOOLEAN DEFAULT FALSE
            )",
            [],
        )?;

        // 笔记版本历史表
        connection.execute(
            "CREATE TABLE IF NOT EXISTS note_versions (
                id TEXT PRIMARY KEY,
                note_id TEXT KEY NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT,
                comment TEXT,
                saved_at TEXT NOT NULL,
                FOREIGN KEY(note_id) REFERENCES notes(id)
            )", 
            [],
        )?;
        Ok( Self { connection } )
    } 

    fn load_note(&self, id: &str) -> Result<Option<Note>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, tags, created_at, updated_at, is_pinned FROM notes
            WHERE id = ?1"
        )?;
        let mut note_iter = stmt.query_map([id], |row| {
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

        match note_iter.next() {
            Some(Ok(note)) => Ok(Some(note)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }

    }

    pub fn load_all_notes(&self) -> Result<Notebook> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, tags, created_at, updated_at, is_pinned FROM notes 
            WHERE is_deleted = FALSE"
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
            notebook.insert_or_replace_note(note);
        }
        Ok(notebook)
    }

    fn insert_or_replace_note(
        tx: &Transaction,
        note: &Note,
    ) -> Result<()> {
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
                note.is_pinned,
            ]
        )?;
        Ok(())
    }

    pub fn save_note(&mut self, note: &Note) -> Result<()> {
        let tx = self.connection.transaction()?;
        Self::insert_or_replace_note(&tx, note)?;
        tx.commit()
    }

    pub fn save_notebook(&mut self, notebook: &Notebook) -> Result<()> {
        let tx = self.connection.transaction()?;
        for note in notebook.notes.values() {
            Self::insert_or_replace_note(&tx, note)?;
        }
        tx.commit()
    }

    // 软删除笔记 （移动到回收站）
    pub fn move_to_trash(&mut self, note_id: &str) -> Result<()> {
        let tx = self.connection.transaction()?;
        tx.execute(
            "UPDATE notes SET is_deleted = TRUE, updated_at = ?1 WHERE id = ?2", 
            rusqlite::params![ Utc::now().to_rfc3339(), note_id ],
        )?;
        tx.commit()
    }

    // 从回收站恢复笔记
    pub fn restore_from_trash(&mut self, note_id: &str) -> Result<Option<Note>> {
        {
            let tx = self.connection.transaction()?;
            tx.execute(
                "UPDATE notes SET is_deleted = FALSE, updated_at = ?1 WHERE id = ?2", 
                rusqlite::params![ Utc::now().to_rfc3339(), note_id ],
            )?;
            tx.commit()?;
        }
        self.load_note(note_id)
    }

    // 永久删除笔记
    pub fn delete_permanently(&mut self, note_id: &str) -> Result<()> {
        let tx = self.connection.transaction()?;
        // 先删除所有版本历史
        tx.execute(
            "DELETE FROM note_versions WHERE note_id = ?1", 
            [note_id]
        )?;

        // 再删除笔记本身
        tx.execute(
            "DELETE FROM notes WHERE id = ?1", 
            [note_id]
        )?;
        tx.commit()
    }

    // 获取回收站中的所有笔记
    pub fn get_deleted_notes(&self) -> Result<Vec<DeleteNote>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, updated_at FROM notes
            WHERE is_deleted = TRUE ORDER BY updated_at DESC",
        )?;

        let notes = stmt.query_map([], |row| {
            Ok(DeleteNote {
                id: row.get(0)?,
                title: row.get(1)?,
                deleted_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?).unwrap().with_timezone(&Utc),
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(notes)
    }

    // 清空回收站
    pub fn empty_trash(&mut self) -> Result<()> {
        // 获取所有已删除笔记的ID
        let deleted_ids = self.connection
            .prepare("SELECT id FROM notes WHERE is_deleted = TRUE")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;

        // 为每个笔记执行彻底删除
        for id in deleted_ids.iter() {
            self.delete_permanently(id)?;
        }

        Ok(())
    }

    // 导出版本历史
    pub fn load_version_history(&self, note_id: &str) -> Result<Vec<NoteVersion>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, note_id, title, content, tags, comment, saved_at FROM note_versions
            WHERE note_id = ?1",
        )?;

        let note_versions = stmt.query_map([note_id], |row| {
            Ok(NoteVersion {
                id: row.get(0)?,
                note_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                comment: row.get(5)?,
                saved_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?).unwrap().with_timezone(&Utc),
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(note_versions)
    }

    fn insert_or_replace_note_version(
        tx: &Transaction,
        note_version: &NoteVersion,
    ) -> Result<()> {
        tx.execute(
            "INSERT OR REPLACE INTO note_versions
            (id, note_id, title, content, tags, comment, saved_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
            rusqlite::params![
                note_version.id,
                note_version.note_id,
                note_version.title,
                note_version.content,
                serde_json::to_string(&note_version.tags).unwrap(),
                note_version.comment,
                note_version.saved_at.to_rfc3339(),
            ]
        )?;
        Ok(())
    }

    pub fn save_version(&mut self, note_version: &NoteVersion) -> Result<()> {
        let tx = self.connection.transaction()?;
        Self::insert_or_replace_note_version(&tx, note_version)?;
        tx.commit()
    }

    pub fn delete_version(&mut self, version_id: &str) -> Result<()> {
        let tx = self.connection.transaction()?;
        tx.execute(
            "DELETE FROM note_versions WHERE id = ?1", 
            [version_id]
        )?;
        tx.commit()
    }
}
