use crate::{
    state::{NoteState, TabState}, 
    i18n::Translate,
};

/// 笔记列表视图
#[derive(Default)]
pub(super) struct NotesView {
    filter: String,
}

impl NotesView {
    pub fn show<T: NoteState + TabState + Translate>(&mut self, ui: &mut egui::Ui, state: &mut T) {
        ui.vertical(|ui| {
            ui.label(state.t("note list"));
            ui.text_edit_singleline(&mut self.filter);

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut to_delete = None;
                for note in state.filter_notes(&self.filter).unwrap() {
                    let is_selected = state.current_note_id_equals(note.id());
                    ui.horizontal(|ui| {
                        let response = ui.selectable_label(
                            is_selected, 
                            note.title()
                        );

                        if response.clicked() {
                            state.load_note(note.id());
                        }

                        // 删除按钮（只在选中时显示）
                        if is_selected {
                            if ui.button("🗑").on_hover_text(&state.t("delete note")).clicked() {
                                to_delete = Some(note.id().to_string());
                            }
                        }
                    });
                }

                // 处理删除操作(在 notebook 锁释放后）
                if let Some(id) = to_delete {
                    if let Err(e) = state.delete_note(&id) {
                        eprintln!("删除笔记失败: {}", e);
                        // 可以在这里添加错误提示到UI
                        ui.label(egui::RichText::new("Failed to delete!").color(egui::Color32::RED));
                    }
                }
            });
        });
    }
}