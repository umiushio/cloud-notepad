use crate::{i18n::Translate, data::DeleteNote, services::TrashService};

/// 标签列表视图
#[derive(Default)]
pub(super) struct TrashView {
    deleted_notes: Vec<DeleteNote>,
    selected_note: Option<usize>,
}

impl TrashView {
    pub fn show<T: TrashService + Translate>(&mut self, ui: &mut egui::Ui, state: &mut T) {
        self.deleted_notes = state.get_deleted_notes().unwrap();

        ui.vertical(|ui| {
            ui.label(state.t("recycle bin"));
            ui.separator();

            // 操作按钮
            ui.horizontal(|ui| {
                if ui.button(state.t("empty trash")).clicked() {
                    let _ = state.empty_trash();
                }
            });

            // 已删除笔记列表
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, note) in self.deleted_notes.iter().enumerate() {
                    let is_selected = self.selected_note == Some(idx);
                    let response = ui.selectable_label(
                        is_selected, 
                        format!(
                            "{} - {}",
                            note.deleted_at(),
                            note.title(),
                        ),
                    );

                    if response.clicked() {
                        self.selected_note = Some(idx);
                    }
                }
            });

            // 选中笔记的操作
            if let Some(idx) = self.selected_note {
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button(state.t("restore")).clicked() {
                        let _ = state.restore_from_trash(self.deleted_notes[idx].id());
                    }

                    if ui.button(state.t("delete permanently")).clicked() {
                        let _ = state.delete_permanently(self.deleted_notes[idx].id());
                    }
                });
            }
        });
    }
}