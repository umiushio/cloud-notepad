use crate::{
    services::{NoteService, TabService, IoService},
    i18n::Translate,
    io::ExportFormat,
};
use super::app_layout::ShowView;
use super::dialogs::file_dialog;

#[derive(Default)]
pub struct MenuBar {

}

impl MenuBar {
    pub fn show<T>(&mut self, ui: &mut egui::Ui, service: &mut T) -> Option<ShowView>
        where T: NoteService + TabService + IoService + Translate {
        let mut result = None;

        egui::menu::bar(ui, |ui| {
            // 笔记
            ui.menu_button(service.t("note"), |ui| {
                // 创建新笔记
                if ui.button(service.t("new note")).clicked() {
                    if let Err(e) = service.create_note() {
                        eprintln!("创建笔记失败: {}", e);
                    }
                    ui.close_menu();
                }
                // 版本历史窗口
                if let Some(_) = service.current_note_id() {
                    ui.separator();
                    if ui.button(service.t("view version history")).clicked() {
                        result = Some(ShowView::ShowVersionHistory);
                        ui.close_menu();
                    }
                }
                ui.separator();
                // 导出菜单项
                ui.menu_button(service.t("export"), |ui| {
                    if let Some(note_id) = service.current_note_id() {
                        if ui.button(service.t("current note")).clicked() {
                            if let Some(path) = match service.export_config().format {
                                ExportFormat::Markdown(_) => file_dialog::save_markdown_file(
                                    service.get_note(note_id).unwrap().title()
                                ),
                                ExportFormat::Json => file_dialog::save_json_file(
                                    service.get_note(note_id).unwrap().title()
                                ),
                                _ => None,
                            }
                            {
                                if let Err(e) = service.export_note(note_id, &path) {
                                    eprintln!("Export Failed: {}", e);
                                }
                            }
                            ui.close_menu();
                        }
                    }

                    if ui.button(service.t("all notes")).clicked() {
                        if let Some(dir) = file_dialog::pick_directroy() {
                            if let Err(e) = service.export_all_notes(&dir) {
                                eprintln!("Export All Notes Failed: {}", e);
                            }
                        }
                        ui.close_menu();
                    }
                });
                // 导入菜单项
                ui.menu_button(service.t("import"), |ui| {
                    if ui.button(service.t("from file")).clicked() {
                        if let Some(path) = file_dialog::pick_available_file() {
                            if let Err(e) = service.import(&path) {
                                eprintln!("Import From File Failed: {}", e);
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button(service.t("from directory")).clicked() {
                        if let Some(dir) = file_dialog::pick_directroy() {
                            if let Err(e) = service.import(&dir) {
                                eprintln!("Import From Directory Failed: {}", e);
                            }
                        }
                        ui.close_menu();   
                    }
                });
                ui.separator();
                // 退出按钮
                if ui.button(service.t("exit")).clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    ui.close_menu();
                }
            });
            // 其他菜单
        });

        result
    }
}