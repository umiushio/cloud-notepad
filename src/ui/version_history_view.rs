use crate::{
    data::NoteVersion,
    services::VersionService,
    i18n::Translate,
};

#[derive(Default)]
pub struct VersionHistoryView {
    is_open: bool,
    selected_note_id: Option<String>,
    versions: Vec<NoteVersion>,
    selected_version: Option<usize>,
    preview_tags: String,
    preview_content: String,
}

impl VersionHistoryView {
    pub fn show<T: VersionService + Translate>(&mut self, ctx: &egui::Context, service: &mut T) -> bool {
        if !self.is_open || self.selected_note_id.is_none() { return false; }
        self.versions = service.list_versions(self.selected_note_id.as_ref().unwrap()).unwrap();

        egui::Window::new("Version Histroy")
            .open(&mut self.is_open)
            .resizable(true)
            .default_width(800.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                egui::SidePanel::left("version_history_sidebar")
                    .resizable(true)
                    .default_width(300.0)
                    .show_inside(ui, |ui| {
                        ui.vertical(|ui| {
                            // 版本列表
                            egui::ScrollArea::vertical()
                                .max_width(300.0)
                                .show(ui, |ui| {
                                    for (idx, version) in self.versions.iter().enumerate() {
                                        let is_selected = self.selected_version == Some(idx);
                                        let response = ui.selectable_label(
                                            is_selected, 
                                        format!(
                                                "{} - {}",
                                                version.saved_at(),
                                                version.comment()
                                            ),
                                        );

                                        if response.clicked() {
                                            self.selected_version = Some(idx);
                                            self.preview_content = version.content().to_string();
                                            self.preview_tags = version.tags().iter().cloned().collect::<Vec<_>>().join(" ");
                                        }
                                    }
                                });
                            // 操作按钮
                            ui.separator();
                            ui.horizontal(|ui| {
                                if let Some(idx) = self.selected_version {
                                    if ui.button(service.t("restore this version")).clicked() {
                                        if let Err(e) = service.restore_version(&self.versions[idx]) {
                                            eprintln!("重载版本失败: {}", e);
                                        }
                                    }

                                    if ui.button(service.t("delete this version")).clicked() {
                                        if let Err(e) = service.delete_version(self.versions[idx].id()) {
                                            eprintln!("删除版本失败: {}", e);
                                        }
                                    }
                                }
                            });
                        });
                    });
                
                egui::CentralPanel::default()
                    .show_inside(ui, |ui| {
                        // 版本详情预览
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if let Some(idx) = self.selected_version {
                                let version = &self.versions[idx];
                                ui.label(version.title());
                                ui.separator();
                                ui.label(&self.preview_tags);
                                ui.separator();
                                ui.label(&self.preview_content);
                            } else if self.versions.is_empty() {
                                ui.label(service.t("no version has been saved"));
                            } else {
                                ui.label(service.t("select a version to preview"));
                            }
                        });
                    });
            });

        self.is_open
    }

    pub fn open(&mut self, note_id: Option<String>) {
        self.is_open = true;
        self.selected_note_id = note_id;
    }
}