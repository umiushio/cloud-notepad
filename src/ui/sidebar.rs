use super::*;

#[derive(Default)]
pub struct NoteSidebar {
    note_filter: String,
}

impl NoteSidebar {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        ui.heading(&state.t("death note"));

        // 搜索框
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.note_filter);
        });

        // 新建笔记按钮
        if ui.button(format!("➕ {}", state.t("add note"))).clicked() {
            state.create_note().unwrap();
        }

        ui.separator();

        // 笔记列表
        let mut to_delete = None;
        let delete_text = &state.t("delete note");
        {
            let notebook = state.notebook.lock().unwrap();
            egui::ScrollArea::vertical().show(ui, |ui| {
                
                for note in notebook.notes.values() {
                    // 过滤笔记
                    if self.note_filter.is_empty() || note.contains(&self.note_filter, None)
                    {
                        let is_selected = state.current_note_id.as_ref() == Some(&note.id);
                    
                        ui.horizontal(|ui| {
                            // 笔记选择按钮
                            if ui.selectable_label(is_selected, &note.title).clicked() {
                                state.current_note_id = Some(note.id.clone());
                            }

                            // 删除按钮（只在选中时显示）
                            if is_selected {
                                if ui.button("🗑").on_hover_text(delete_text).clicked() {
                                    to_delete = Some(note.id.clone());
                                }
                            }
                            
                        });
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
    }
}