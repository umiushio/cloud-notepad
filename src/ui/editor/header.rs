use crate::{data::Note, i18n::Translate};

#[derive(Default)]
pub(super) struct EditorHeader {
    // title_edit: String,
    // is_editing_title: bool,
    tag_edit: String,
}

impl EditorHeader {
    pub fn show<T: Translate>(&mut self, ui: &mut egui::Ui, note: &mut Note, t: &T) -> bool {
        let mut flag = false;
        ui.vertical(|ui| {
            // // 标题编辑区域
            // flag |= self.show_title(ui, note);

            // 标签编辑区域
            flag |= self.show_tags(ui, note, t);
        });
        flag
    }

    // fn show_title(&mut self, ui: &mut egui::Ui, note: &mut Note) -> bool {
    //     let mut flag = false;
    //     ui.horizontal(|ui| {
    //         if self.is_editing_title {
    //             ui.text_edit_singleline(&mut self.title_edit);
    //             if ui.button("✔").clicked() || ui.input(
    //                 |i| i.key_pressed(egui::Key::Enter)
    //             ) {
    //                 if self.title_edit.is_empty() {
    //                     self.title_edit = "untitled".to_string();
    //                 }
    //                 // 更新标题，若变化时需要通知state更新笔记
    //                 if note.update_title(self.title_edit.clone()) {
    //                     flag = true;
    //                 }
    //                 self.is_editing_title = false;
    //             }
    //         } else {
    //             ui.heading(note.title());
    //             if ui.button("✏").clicked() {
    //                 self.title_edit = note.title().to_string();
    //                 self.is_editing_title = true;
    //             }
    //         }
    //     });

    //     flag
    // }

    fn show_tags<T: Translate>(&mut self, ui: &mut egui::Ui, note: &mut Note, t: &T) -> bool {
        let mut flag = false;
        ui.vertical(|ui| {
            // 标签编辑
            ui.horizontal(|ui| {
                egui::TextEdit::singleline(&mut self.tag_edit).hint_text(t.t("new tag")).show(ui);
                if ui.button("+").clicked() && !self.tag_edit.is_empty() {
                    note.add_tag(self.tag_edit.clone());
                    flag = true;
                    self.tag_edit.clear();
                }
            });

            // 显示现有标签
            ui.horizontal_wrapped(|ui| {
                for tag in note.tags().iter() {
                    ui.label(tag);
                }
            })
        });

        flag
    }
}