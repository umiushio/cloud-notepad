use crate::data::DeleteNote;
use super::AppState;
use super::TabService;

pub trait TrashService {
    fn move_to_trash(&mut self, note_id: &str) -> anyhow::Result<()>;
    fn restore_from_trash(&mut self, note_id: &str) -> anyhow::Result<()>;
    fn empty_trash(&mut self) -> anyhow::Result<()>;
    fn delete_permanently(&mut self, note_id: &str) -> anyhow::Result<()>;
    fn get_deleted_notes(&self) -> anyhow::Result<Vec<DeleteNote>>;
}

impl TrashService for AppState {
    /// 删除指定笔记
    fn move_to_trash(&mut self, note_id: &str) -> anyhow::Result<()> {
        {
            let mut notebook = self.notebook.lock().unwrap();
            // 从内存中移除
            notebook.delete_note(note_id);

            // 从数据库中改变删除状态
            let mut conn = self.db_conn.lock().unwrap();
            conn.move_to_trash(note_id)?;
        }
        // 清除相关状态
        self.close_note(note_id);

        Ok(())
    }

    fn restore_from_trash(&mut self, note_id: &str) -> anyhow::Result<()> {
        // 先从从数据库中恢复
        let mut conn = self.db_conn.lock().unwrap();
        let note = conn.restore_from_trash(note_id)?.unwrap();

        // 再从内存中恢复
        let mut notebook = self.notebook.lock().unwrap();
        notebook.insert_or_replace_note(note);
        Ok(())
    }

    fn empty_trash(&mut self) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.empty_trash()?;
        Ok(())
    }

    fn delete_permanently(&mut self, note_id: &str) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.delete_permanently(note_id)?;
        Ok(())
    }

    fn get_deleted_notes(&self) -> anyhow::Result<Vec<DeleteNote>> {
        let conn = self.db_conn.lock().unwrap();
        let deleted_notes = conn.get_deleted_notes()?;
        Ok(deleted_notes)
    }
}