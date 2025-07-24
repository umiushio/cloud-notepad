#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_note_crud() {
        // 使用临时数据库
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        
        // 初始化测试数据
        let mut notebook = Notebook::default();
        let mut note = Note::new("测试笔记");
        note.content = "测试内容".to_string();
        notebook.add_note(note.clone());
        
        // 测试保存
        notebook.save_to_db(&conn).unwrap();
        
        // 测试加载
        let loaded = Notebook::load_from_db(&conn).unwrap();
        assert_eq!(loaded.notes.len(), 1);
        assert_eq!(loaded.notes[&note.id].title, "测试笔记");
        
        // 测试更新
        let mut updated_note = note.clone();
        updated_note.title = "修改后的标题".to_string();
        notebook.add_note(updated_note.clone());
        notebook.save_to_db(&conn).unwrap();
        
        // 测试删除
        notebook.notes.remove(&note.id);
        notebook.save_to_db(&conn).unwrap();
        let after_delete = Notebook::load_from_db(&conn).unwrap();
        assert_eq!(after_delete.notes.len(), 0);
    }

    #[test]
    fn test_state_partial_save() {
        let state = AppState::new().unwrap();
        
        // 添加测试笔记
        let mut note = Note::new("测试笔记");
        state.notebook.lock().unwrap().add_note(note.clone());
        
        // 标记为当前笔记并修改
        state.current_note_id = Some(note.id.clone());
        state.mark_note_modified(&note.id);
        
        // 测试部分保存
        state.save_current_note().unwrap();
        
        // 验证数据库
        let conn = state.db_conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE id = ?1", 
            [note.id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(count, 1);
    }
}