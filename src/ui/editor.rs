use super::*;

#[derive(Default)]
pub struct NoteEditor {
    title_edit: String,
    is_editing_title: bool,
    pub md_content: String,
}

impl NoteEditor {
    pub fn show(&mut self, ui: &mut egui::Ui, note: &mut Note) {
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

        // Markdown编辑区域
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.md_content != note.content {
                self.md_content = note.content.clone();
            }

            let response = ui.add(
                egui::TextEdit::multiline(&mut self.md_content)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
            );

            if response.changed() {
                note.content = self.md_content.clone();
                note.updated_at = chrono::Utc::now();
            }
        });

        // 状态栏
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("words: ");
            ui.label(note.content.chars().count().to_string());
            ui.label("last updated at: ");
            ui.label(note.updated_at.format("%Y-%m-%d %H:%M").to_string());
        });
    }
}