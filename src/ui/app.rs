use super::*;

pub struct NoteApp {
    state: AppState,
}

impl NoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(cc),
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Cloud Notes");

            // 临时显示笔记数量
            {
                let notebook = self.state.notebook.lock().unwrap();
                ui.label(format!("Total notes: {}", notebook.notes.len()));
            }

            // 添加新笔记按钮
            if ui.button("Add New Note").clicked() {
                let mut notebook = self.state.notebook.lock().unwrap();
                let new_note = Note::new("Untitled".to_string());
                notebook.add_note(new_note);

                // 保存到数据库
                let mut conn = self.state.db_conn.lock().unwrap();
                conn.save_notebook(&notebook).expect("Failed to save note");
            }

        });
    }
}