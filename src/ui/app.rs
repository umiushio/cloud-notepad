use super::*;

pub struct NoteApp {
    state: AppState,
    main_window: MainWindow,
}

impl NoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> anyhow::Result<Self> {
        let state = AppState::new()?;

        // 设置初始主题
        if state.dark_mode {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
        }

        Ok(Self {
            state,
            main_window: MainWindow::default(),
        })
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理快捷键
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            if let Err(e) = self.state.save_all_note() {
                eprintln!("保存全部笔记失败: {}", e);
            } 
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            if let Err(e) = self.state.delete_current_note() {
                eprintln!("删除当前笔记失败: {}", e);
            }
        }
        
        self.main_window.show(ctx, &mut self.state);

        // 自动保存检查
        if self.state.current_note_is_modified() {
            if let Err(e) = self.state.save_current_note() {
                eprintln!("自动保存失败: {}", e);
            }
        }
    }

    // fn on_close_event(&mut self) -> bool {
    //     if self.state.needs_save {
    //         if let Err(e) = self.state.save_changes() {
    //             eprintln!("退出前保存失败: {}", e);
    //         }
    //     }
    //     true
    // }
}