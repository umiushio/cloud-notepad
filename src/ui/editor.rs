use super::*;

/// 笔记编辑器状态
#[derive(Default)]
pub struct NoteEditor {
    title_edit: String,
    is_editing_title: bool,
    md_editor: MarkdownEditor,
    status_bar: StatusBar,
}

impl NoteEditor {
    pub fn show<T: Translate>(&mut self, ui: &mut egui::Ui, note: &mut Note, t: &T) {
        // 标题编辑区域
        ui.horizontal(|ui| {
            if self.is_editing_title {
                ui.text_edit_singleline(&mut self.title_edit);
                if ui.button("✔").clicked() || ui.input(
                    |i| i.key_pressed(egui::Key::Enter)
                ) {
                    note.title = self.title_edit.clone();
                    self.is_editing_title = false;
                }
            } else {
                ui.heading(&note.title);
                if ui.button("✏").clicked() {
                    self.title_edit = note.title.clone();
                    self.is_editing_title = true;
                }
            }
        });

        ui.separator();

        // Markdown编辑器区域
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut text = note.content.clone();
            self.md_editor.show(ui, &mut text, t);
            if note.content != text {
                note.content = text;
                note.updated_at = chrono::Utc::now();
            }
        });

        // 底部状态栏
        self.status_bar.show(ui, note.content.chars().count(), note.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(), t);
    }
}