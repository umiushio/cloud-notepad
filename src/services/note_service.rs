use crate::data::Note;
use super::AppState;
use super::{TabService, TrashService};

pub trait NoteService {
    fn create_note(&mut self) -> anyhow::Result<()>;
    fn update_note(&mut self, note: Note) -> anyhow::Result<()>;
    fn delete_note(&mut self, note_id: &str) -> anyhow::Result<()>;
    fn save_note(&self, note: &Note) -> anyhow::Result<()>;
    fn get_note(&self, note_id: &str) -> Option<Note>;
    fn filter_notes(&self, key: &str) -> anyhow::Result<Vec<Note>>;
}

impl NoteService for AppState {
    /// 新建笔记
    fn create_note(&mut self) -> anyhow::Result<()> {
        let new_note = Note::new("untitled".to_string());
        self.load_note(new_note.id());
        self.update_note(new_note)
    }

    /// 获取笔记
    fn get_note(&self, note_id: &str) -> Option<Note> {
        self.notebook.lock().unwrap().find_note(note_id)
    }

    // 更新笔记
    fn update_note(&mut self, note: Note) -> anyhow::Result<()> {
        let note_id = note.id().to_string();
        {
            let mut notebook = self.notebook.lock().unwrap();
            notebook.insert_or_replace_note(note);
        }
        self.flush_modified_note(Some(note_id))?;
        Ok(())
    }

    /// 保存指定笔记
    fn save_note(&self, note: &Note) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.save_note(note)?;
        Ok(())
    }

    /// 删除笔记
    fn delete_note(&mut self, note_id: &str) -> anyhow::Result<()> {
        self.move_to_trash(note_id)
    }

    /// 筛选笔记
    fn filter_notes(&self, key: &str) -> anyhow::Result<Vec<Note>> {
        Ok(self.notebook.lock().unwrap().filter_notes(key)
            .into_iter()
            .cloned()
            .collect())
    }
}