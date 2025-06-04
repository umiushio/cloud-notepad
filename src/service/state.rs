use super::*;
use anyhow::Ok;
use std::{collections::HashSet, sync::{Arc, Mutex}};


pub struct AppState {
    db_conn: Arc<Mutex<Database>>,
    pub(crate) notebook: Arc<Mutex<Notebook>>,
    pub(crate) current_note_id: Option<String>,
    pub(crate) dark_mode: bool,
    modified_notes: Mutex<HashSet<String>>,
    pub(crate) current_language: Language,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        // 初始化数据库连接并加载初始数据
        let db = Database::new()?;
        let notebook = db.load_all_notes()?;

        Ok(Self {
            db_conn: Arc::new(Mutex::new(db)),
            notebook: Arc::new(Mutex::new(notebook)),
            current_note_id: None,
            dark_mode: true,
            modified_notes: Mutex::new(HashSet::new()),
            current_language: Language::English,
        })
    }

    pub fn current_note(&self) -> Option<Note> {
        let notebook = self.notebook.lock().unwrap();
        self.current_note_id
            .as_ref()
            .and_then(|id| notebook.notes.get(id))
            .cloned()
    }

    pub fn mark_note_modified(&self, note_id: String) {
        self.modified_notes.lock().unwrap().insert(note_id);
    }

    pub fn unmark_note_modified(&self, note_id: &String) {
        self.modified_notes.lock().unwrap().remove(note_id);
    }

    pub fn clear_note_modified(&self) {
        self.modified_notes.lock().unwrap().clear();
    }

    pub fn current_note_is_modified(&self) -> bool {
        if let Some(note_id) = &self.current_note_id {
            return self.modified_notes.lock().unwrap().contains(note_id);
        }
        return false;
    }

    /// 新建笔记，并自动保存
    pub fn create_note(&mut self) -> anyhow::Result<()> {
        let new_note = Note::new("untitled".to_string());
        let note_id = new_note.id.clone();
        self.save_note(&new_note)?;
        let mut notebook = self.notebook.lock().unwrap();
        notebook.add_note(new_note);
        self.current_note_id = Some(note_id);
        Ok(())
    }

    // 更新笔记，不会保存
    pub fn update_note(&mut self, note: Note) -> anyhow::Result<()> {
        let note_id = note.id.clone();
        let mut notebook = self.notebook.lock().unwrap();
        notebook.add_note(note);
        self.current_note_id = Some(note_id.clone());
        self.mark_note_modified(note_id);
        Ok(())
    }

    /// 保存指定笔记
    pub fn save_note(&self, note: &Note) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.save_note(note)?;
        self.unmark_note_modified(&note.id);
        Ok(())
    }

    /// 保存当前修改过的笔记，会做修改检查
    pub fn save_current_note(&self) -> anyhow::Result<()> {
        let note_id = match &self.current_note_id {
            Some(id) => id,
            None => return Ok(()),
        };
        
        let notebook = self.notebook.lock().unwrap();
        if let Some(note) = notebook.notes.get(note_id) {
            let mut conn = self.db_conn.lock().unwrap();
            conn.save_note(note)?;
            self.unmark_note_modified(note_id);
        }
        Ok(())
    }

    /// 全量保存
    pub fn save_all_note(&mut self) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        let notebook = self.notebook.lock().unwrap();
        conn.save_notebook(&notebook)?;
        self.clear_note_modified();
        Ok(())
    }

    /// 删除指定笔记
    pub fn delete_note(&mut self, note_id: &String) -> anyhow::Result<()> {
        let mut notebook = self.notebook.lock().unwrap();
        // 从内存中移除
        notebook.delete_note(note_id);

        // 从数据库中删除
        let mut conn = self.db_conn.lock().unwrap();
        conn.delete_note(note_id)?;

        // 清除相关状态
        if self.current_note_id.as_ref() == Some(&note_id) {
            self.current_note_id = None;
        }
        self.unmark_note_modified(note_id);

        Ok(())
    }

    /// 删除当前笔记
    pub fn delete_current_note(&mut self) -> anyhow::Result<()> {
        if let Some(note_id) = &self.current_note_id {
            self.delete_note(&note_id.clone())
        } else {
            Ok(())
        }
    }

    /// 文本转换
    pub fn t(&self, key: &str) -> String {
        i18n::t(key, self.current_language)
    }
}