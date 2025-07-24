use crate::data::Note;
use crate::message::sync::SyncMessage;
use crate::message::Message;
use super::AppState;
use super::{TabState, TrashState};

pub trait NoteState {
    fn create_note(&mut self, title: &str) -> anyhow::Result<()>;
    fn update_note(&mut self, note: Note) -> anyhow::Result<()>;
    fn rewrite_note(&mut self, note: Note) -> anyhow::Result<()>;
    fn delete_note(&mut self, note_id: &str) -> anyhow::Result<()>;
    fn save_note(&self, note: &Note) -> anyhow::Result<()>;
    fn get_note(&self, note_id: &str) -> Option<Note>;
    fn get_note_by_title(&self, title: &str) -> Vec<Note>;
    fn filter_notes(&self, key: &str) -> anyhow::Result<Vec<Note>>;
    fn reload_notes(&self, user_id: &str) -> anyhow::Result<()>;
}

impl NoteState for AppState {
    /// 新建笔记
    fn create_note(&mut self, title: &str) -> anyhow::Result<()> {
        let title = if title.is_empty() { "untitled".to_string() } else { title.to_string() };
        let user_id = self.user_id.clone();
        let new_note = Note::new(&title, &user_id);
        println!("create note...");
        // 1. 内存以及本地创建笔记
        self.rewrite_note(new_note.clone())?;
        self.load_note(new_note.id());      // 载入笔记

        println!("cloud sync create note");
        // 2. 云端同步
        if self.user_name.is_some() {
            let sender = self.sender.clone();
            if let Err(e) = sender.try_send(Message::SyncMessage(SyncMessage::CreateNote {
                id: new_note.id().to_string(),
                title,
                created_at: new_note.created_at()
            })) {
                eprintln!("send create message failed: {}", e);
            }
        }
        Ok(())
    }

    /// 获取笔记
    fn get_note(&self, note_id: &str) -> Option<Note> {
        self.notebook.lock().unwrap().find_note(note_id)
    }

    /// 通过标题获取笔记
    fn get_note_by_title(&self, title: &str) -> Vec<Note> {
        let notebook = self.notebook.lock().unwrap();
        notebook.find_notes_by_title(title)
            .into_iter()
            .cloned()
            .collect()
    }

    /// 重写笔记，会更新本地存储，且不会触发防抖逻辑
    fn rewrite_note(&mut self, note: Note) -> anyhow::Result<()> {
        {
            let mut notebook = self.notebook.lock().unwrap();
            notebook.insert_or_replace_note(note.clone());
        }
        self.save_note(&note)?;
        Ok(())
    }

    // 更新笔记，不会更新本地存储，且会触发防抖逻辑
    fn update_note(&mut self, note: Note) -> anyhow::Result<()> {
        let note_id = note.id().to_string();
        {
            let mut notebook = self.notebook.lock().unwrap();
            notebook.insert_or_replace_note(note);
        }
        // 记录防抖信息
        self.debounce_modified.insert(note_id);
        self.debounce_last_edit = Some(std::time::Instant::now());
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
        self.move_to_trash(note_id)?;
        // 云端同步
        if self.user_name.is_some() {
            let sender = self.sender.clone();
            if let Err(e) = sender.try_send(Message::SyncMessage(SyncMessage::DeleteNote { id: note_id.to_string() } )) {
                eprintln!("send delete message failed: {}", e);
            }
        }
        Ok(())
    }

    /// 筛选笔记
    fn filter_notes(&self, key: &str) -> anyhow::Result<Vec<Note>> {
        Ok(self.notebook.lock().unwrap().filter_notes(key)
            .into_iter()
            .cloned()
            .collect())
    }

    // 重载笔记
    fn reload_notes(&self, user_id: &str) -> anyhow::Result<()> {
        let conn = self.db_conn.lock().unwrap();
        let mut notebook = self.notebook.lock().unwrap();
        *notebook = conn.load_all_notes(user_id)?;
        Ok(())
    }
}