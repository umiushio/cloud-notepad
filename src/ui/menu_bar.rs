use crate::{
    i18n::Translate, io::ExportFormat, state::{IoState, NoteState, TabState}
};
use super::app_layout::ShowView;
use super::dialogs::{file_dialog, singleline_dialog::SinglelineDialog};

pub struct MenuBar {
    title_dialog: SinglelineDialog,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self { 
            title_dialog: SinglelineDialog::new(
                "new title", 
                "enter new title", 
                "", 
                "title cannot be empty!",
            ),
        }
    }
}

impl MenuBar {
    pub fn show<T>(&mut self, ui: &mut egui::Ui, state: &mut T) -> Option<ShowView>
        where T: NoteState + TabState + IoState + Translate {
        let mut result = ShowView::default();
        let mut show_view = false;

        egui::menu::bar(ui, |ui| {
            // 笔记
            ui.menu_button(state.t("note"), |ui| {
                // 创建新笔记
                if ui.button(state.t("new note")).clicked() {
                    self.title_dialog.open();
                    ui.close_menu();
                }
                // 版本历史窗口
                if let Some(_) = state.current_note_id() {
                    ui.separator();
                    if ui.button(state.t("view version history")).clicked() {
                        result.show_version_history = true;
                        show_view = true;
                        ui.close_menu();
                    }
                }
                ui.separator();
                // 导出菜单项
                ui.menu_button(state.t("export"), |ui| {
                    if let Some(note_id) = state.current_note_id() {
                        if ui.button(state.t("current note")).clicked() {
                            if let Some(path) = match state.export_config().format {
                                ExportFormat::Markdown(_) => file_dialog::save_markdown_file(
                                    state.get_note(note_id).unwrap().title()
                                ),
                                ExportFormat::Json => file_dialog::save_json_file(
                                    state.get_note(note_id).unwrap().title()
                                ),
                                _ => None,
                            }
                            {
                                if let Err(e) = state.export_note(note_id, &path) {
                                    eprintln!("Export Failed: {}", e);
                                }
                            }
                            ui.close_menu();
                        }
                    }

                    if ui.button(state.t("all notes")).clicked() {
                        if let Some(dir) = file_dialog::pick_directroy() {
                            if let Err(e) = state.export_all_notes(&dir) {
                                eprintln!("Export All Notes Failed: {}", e);
                            }
                        }
                        ui.close_menu();
                    }
                });
                // 导入菜单项
                ui.menu_button(state.t("import"), |ui| {
                    if ui.button(state.t("from file")).clicked() {
                        if let Some(path) = file_dialog::pick_available_file() {
                            if let Err(e) = state.import(&path) {
                                eprintln!("Import From File Failed: {}", e);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button(state.t("from directory")).clicked() {
                        if let Some(dir) = file_dialog::pick_directroy() {
                            if let Err(e) = state.import(&dir) {
                                eprintln!("Import From Directory Failed: {}", e);
                            }
                        }
                        ui.close_menu();   
                    }
                });
                ui.separator();
                // 退出按钮
                if ui.button(state.t("exit")).clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    ui.close_menu();
                }
            });
            // 其他菜单
            ui.menu_button(state.t("help"), |ui| {
                // 打开日志视图
                if ui.button(state.t("logs")).clicked() {
                    result.show_logs = true;
                    show_view = true;
                    ui.close_menu();
                }
            });

            // 弹出窗口
            if let Some(title) = self.title_dialog.show(ui.ctx(), state) {
                if let Err(e) = state.create_note(&title) {
                    eprintln!("创建笔记失败: {}", e);
                }
            }
        });
        
        if show_view {
            Some(result)
        } else {
            None
        }
    }
}