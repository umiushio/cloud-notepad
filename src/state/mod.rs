pub mod note;
pub mod tab;
pub mod trash;
pub mod version;
pub mod settings;
pub mod io;
pub mod auth;
pub mod sync;
pub mod log;

use anyhow::Ok;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use std::{collections::HashSet, sync::{atomic::AtomicBool, Arc, Mutex}, time::Instant};
use crate::{
    data::{Database, Notebook}, i18n::{self, Language, Translate}, io::{ExportConfig, ImportConfig}, message::Message, logger::memory::MemoryLogger, utils::tab_manager::TabManager
};
pub use {
    note::NoteState,
    tab::TabState,
    version::VersionState,
    trash::TrashState,
    settings::SettingsState,
    io::IoState,
    auth::AuthState,
    sync::SyncState,
};


pub struct AppState {
    db_conn: Arc<Mutex<Database>>,
    notebook: Arc<Mutex<Notebook>>,
    recent_notes: TabManager<String>,
    pub(crate) debounce_modified: HashSet<String>,
    pub(crate) debounce_last_edit: Option<Instant>,
    export_config: ExportConfig,
    import_config: ImportConfig,
    theme: Theme,
    language: Language,
    sender: mpsc::Sender<Message>,
    is_authenticated: Arc<AtomicBool>,
    user_name: Option<String>,
    user_id: String,
    last_sync_time: Arc<Mutex<Option<DateTime<Utc>>>>,
    sync_cancel_token: Option<CancellationToken>,
    memory_logger: Arc<MemoryLogger>,
}

impl AppState {
    pub fn new(sender: mpsc::Sender<Message>, memory_logger: MemoryLogger) -> anyhow::Result<Self> {
        // 初始化数据库连接并加载初始数据
        let db = Database::new()?;
        let notebook = db.load_all_notes("")?;

        Ok(Self {
            db_conn: Arc::new(Mutex::new(db)),
            notebook: Arc::new(Mutex::new(notebook)),
            recent_notes: TabManager::new(7),
            debounce_modified: HashSet::new(),
            debounce_last_edit: None,
            export_config: ExportConfig::default(),
            import_config: ImportConfig::default(),
            theme: Theme::Dark,
            language: Language::English,
            sender,
            is_authenticated: Arc::new(AtomicBool::new(false)),
            user_name: None,
            user_id: String::new(),
            last_sync_time: Arc::new(Mutex::new(None)),
            sync_cancel_token: None,
            memory_logger: Arc::new(memory_logger),
        })
    }
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