pub mod note_service;
pub mod tab_service;
pub mod trash_service;
pub mod version_service;
pub mod settings_service;
pub mod io_service;

use anyhow::Ok;
use std::sync::{Arc, Mutex};
use crate::{
    data::{Database, Notebook},
    io::{ExportConfig, ImportConfig}, 
    i18n::{self, Language, Translate}, 
    utils::tab_manager::TabManager
};
pub use {
    note_service::NoteService,
    tab_service::TabService,
    version_service::VersionService,
    trash_service::TrashService,
    settings_service::SettingsService,
    io_service::IoService,
};


pub struct AppState {
    db_conn: Arc<Mutex<Database>>,
    notebook: Arc<Mutex<Notebook>>,
    recent_notes: TabManager<String>,
    modified_note: Arc<Mutex<Option<String>>>,
    export_config: ExportConfig,
    import_config: ImportConfig,
    theme: Theme,
    language: Language,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        // 初始化数据库连接并加载初始数据
        let db = Database::new()?;
        let notebook = db.load_all_notes()?;

        Ok(Self {
            db_conn: Arc::new(Mutex::new(db)),
            notebook: Arc::new(Mutex::new(notebook)),
            recent_notes: TabManager::new(7),
            modified_note: Arc::new(Mutex::new(None)),
            export_config: ExportConfig::default(),
            import_config: ImportConfig::default(),
            theme: Theme::Dark,
            language: Language::English,
        })
    }

    /// 刷新修改笔记
    fn flush_modified_note(&self, note_id: Option<String>) -> anyhow::Result<()> {
        let mut modified_note = self.modified_note.lock().unwrap();
        if let Some(id) = modified_note.take() {
            if let Some(modified_note) = self.notebook.lock().unwrap().find_note(&id) {
                self.save_note(&modified_note)?;
            }
        }
        *modified_note = note_id;
        Ok(())
    }

    // /// 保存当前修改过的笔记，会做修改检查
    // pub fn save_current_note(&mut self) -> anyhow::Result<()> {
    //     if let Some(note_id) = self.current_note_id() {
    //         let notebook = self.notebook.lock().unwrap();
    //         if let Some(note) = notebook.find_note(note_id) {
    //             let mut conn = self.db_conn.lock().unwrap();
    //             conn.save_note(&note)?;
    //         }
    //     }
    //     Ok(())
    // }

    // /// 全量保存
    // pub fn save_all_note(&mut self) -> anyhow::Result<()> {
    //     let mut conn = self.db_conn.lock().unwrap();
    //     let notebook = self.notebook.lock().unwrap();
    //     conn.save_notebook(&notebook)?;
    //     *self.modified_note.lock().unwrap() = None;
    //     Ok(())
    // }
}

impl Translate for AppState {
    fn t(&self, key: &str) -> String {
        i18n::t(key, self.language)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}