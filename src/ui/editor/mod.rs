mod tabs;
mod header;
mod body;
mod preview;

use tabs::EditorTabs;
use header::EditorHeader;
use body::EditorBody;
use super::dialogs::singleline_dialog::SinglelineDialog;

use crate::{services::{NoteService, TabService, VersionService}, i18n::Translate};

pub struct EditorPanel {
    tabs: EditorTabs,
    header: EditorHeader,
    body: EditorBody,
    save_version_dialog: SinglelineDialog,
}

impl Default for EditorPanel {
    fn default() -> Self {
        Self {
            tabs: EditorTabs::default(),
            header: EditorHeader::default(),
            body: EditorBody::default(),
            save_version_dialog: SinglelineDialog::new(
                "save version", 
                "enter version comment", 
                "what's changed in this version?", 
                "comment cannot be empty!",
            ),
        }
    }
}

impl EditorPanel {
    pub fn show<T>(&mut self, ui: &mut egui::Ui, service: &mut T) 
        where T: NoteService + TabService + VersionService + Translate {
        if let Some(mut note) = service.current_note() {
            // 检测快捷键
            self.check_shortcut(ui.ctx());
            // 显示编辑区
            let mut flag = false;
            ui.vertical(|ui| {
                // 笔记页签栏 + 工具栏
                self.tabs.show(ui, service);

                // 标题和标签编辑区
                flag |= self.header.show(ui, &mut note, service); 

                // 编辑/预览区域
                flag |= self.body.show(
                    ui, 
                    &mut note, 
                    self.tabs.show_preview, 
                    self.tabs.split_view,
                );
            });
            if flag {
                if let Err(e) = service.update_note(note.clone()) {
                    eprintln!("更新笔记失败: {}", e);
                    // 可以在这里添加错误提示到UI
                    ui.label(egui::RichText::new("Failed to delete!").color(egui::Color32::RED));
                }
            }
            if let Some(comment) = self.save_version_dialog.show(ui.ctx(), service) {
                if let Err(e) = service.save_version(&comment, &note) {
                    eprintln!("保存版本失败: {}", e);
                }
            }
        } else {
            ui.label(service.t("please select a note from the sidebar or create a new note"));
        }
    }

    // 检测快捷键
    fn check_shortcut(&mut self, ctx: &egui::Context) {
        // 检测 Ctrl+Shift+S
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) {
            self.save_version_dialog.open();
        }
    }
}