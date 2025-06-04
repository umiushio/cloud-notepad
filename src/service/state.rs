use super::*;

use std::sync::{Arc, Mutex};


pub struct AppState {
    pub db_conn: Arc<Mutex<Database>>,
    pub notebook: Arc<Mutex<Notebook>>,
    pub current_note_id: Option<String>,
    pub dark_mode: bool,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 初始化数据库连接
        let db = Database::new().expect("Failed to connect to database");
        // 加载初始数据
        let notebook = db.load_all_notes().expect("Failed to load notes");
        
        let db_conn = Arc::new(Mutex::new(db));
        let notebook = Arc::new(Mutex::new(notebook));

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