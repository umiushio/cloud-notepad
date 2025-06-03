use std::sync::{Arc, Mutex};
use crate::data::model::Notebook;
use crate::data::db::{DbConnection, establish_connection}; // Add this import or adjust the path as needed

pub struct AppState {
    pub db_conn: Arc<Mutex<DbConnection>>,
    pub notebook: Arc<Mutex<Notebook>>,
    pub current_note_id: Option<String>,
    pub dark_mode: bool,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 初始化数据库连接
        let db_conn = Arc::new(Mutex::new(
            establish_connection().expect("Failed to connect to database")
        ));

        // 加载初始数据
        let notebook = Arc::new(Mutex::new(
            Notebook::load_from_db(&mut db_conn.lock().unwrap())
                .expect("Failed to load notes")
        ));

        // 设置默认主题
        let dark_mode = true;
        if dark_mode {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
        }

        Self {
            db_conn,
            notebook,
            current_note_id: None,
            dark_mode
        }
    }
}