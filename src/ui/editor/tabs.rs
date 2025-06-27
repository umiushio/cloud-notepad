use crate::{
    ui::dialogs::singleline_dialog::SinglelineDialog,
    services::{NoteService, TabService},
    i18n::Translate,
};


pub(super) struct EditorTabs {
    pub(super) show_preview: bool,
    pub(super) split_view: bool,
    selected_note_id: Option<String>,
    title_dialog: SinglelineDialog,
}

impl Default for EditorTabs {
    fn default() -> Self {
        Self {
            show_preview: true,
            split_view: false,
            selected_note_id: None,
            title_dialog: SinglelineDialog::new(
                "rename title", 
                "enter new title", 
                "", 
                "title cannot be empty!",
            ),
        }
    }
}

impl EditorTabs {
    pub fn show<T: NoteService + TabService + Translate>(&mut self, ui: &mut egui::Ui, service: &mut T) {
        ui.horizontal(|ui| {
            // 文件标签
            let recent_note_ids: Vec<String> = service.recent_notes().iter().map(|&s| s.to_owned()).collect();
            if !recent_note_ids.is_empty() {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing.x = 10.0;    // 紧凑布局

                    for note_id in recent_note_ids.iter() {
                        if let Some(note) = service.get_note(note_id) {
                            let tab_response = ui.add(
                                egui::Button::new(note.title())
                                    .fill(if service.current_note_id_equals(note_id) {
                                        ui.visuals().widgets.active.bg_fill
                                    } else {
                                        ui.visuals().widgets.inactive.bg_fill
                                    }) 
                                    .corner_radius(0.0)
                                    .frame(false)
                            );

                            if tab_response.clicked() {
                                service.load_note(note_id);
                            }

                            // 右键菜单
                            tab_response.context_menu(|ui| {
                                if ui.button(service.t("close")).clicked() {
                                    service.close_note(note_id);
                                    ui.close_menu();
                                }
                                ui.separator();
                                if ui.button(service.t("rename")).clicked() {
                                    self.selected_note_id = Some(note_id.clone());
                                    // 打开重命名对话框
                                    self.title_dialog.open();
                                    self.title_dialog.set_input(note.title());
                                    ui.close_menu();
                                }
                            });

                            // 关闭按钮
                            if ui.add(
                                egui::Button::new("×")
                                    .frame(false)
                                    .small()
                            ).clicked() {
                                service.close_note(note_id);
                            }
                        }
                    }
                });
            }

            // 工具栏 (右侧)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.toggle_value(&mut self.split_view, "🖽");    // 分屏图标
                ui.toggle_value(&mut self.show_preview, "👁");  // 预览图标
                ui.separator();
            })
        });
        if let Some(title) = self.title_dialog.show(ui.ctx(), service) {
            if let Some(note_id) = &self.selected_note_id {
                if let Some(mut note) = service.get_note(note_id) {
                    if note.update_title(title) {
                        if let Err(e) = service.update_note(note.clone()) {
                            eprintln!("更新笔记标题失败: {}", e);
                            // 可以在这里添加错误提示到UI
                            ui.label(egui::RichText::new("Failed to update note title!").color(egui::Color32::RED));
                        }
                    }
                }
            }
        }
    }
}