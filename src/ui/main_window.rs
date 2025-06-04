use super::*;
use crate::i18n::Language;

#[derive(Default)]
pub struct MainWindow {
    editor: NoteEditor,
    sidebar: NoteSidebar,
}

impl MainWindow {
    pub fn show(&mut self, ctx: &egui::Context, state: &mut AppState) {
        // 语言切换按钮
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("English").clicked() {
                    state.current_language = Language::English;
                }
                if ui.button("中文").clicked() {
                    state.current_language = Language::Chinese;
                }
                if ui.button("日本語").clicked() {
                    state.current_language = Language::Japanese;
                }
            });
        });

        // 侧边栏
        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                self.sidebar.show(ui, state);
            });
        
        // 编辑页面
        egui::CentralPanel::default().show(ctx, |ui| {
            if state.current_note_id.is_none() {
                ui.label(&state.t("please select a note from the sidebar or create a new note"));
            } else {
                self.editor.show(ui, state);
            }
        });
    }
}