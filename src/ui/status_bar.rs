use crate::{services::TabService, i18n::Translate};

#[derive(Default)]
pub struct StatusBar {

}

impl StatusBar {
    pub fn show<T: TabService + Translate>(&self, ui: &mut egui::Ui, service: &T) {
        ui.horizontal(|ui| {
            // 左侧：账户信息
            // ...
            ui.label(service.t("guest"));

            ui.separator();

            // 中间: 笔记信息
            if let Some(note) = service.current_note() {
                let words = note.content().split_whitespace().count();
                let chars = note.content().chars().count();

                ui.label(format!(
                    "📄 {} | {}: {} | {}: {} | {}: {}",
                    note.title(),
                    service.t("words"), words,
                    service.t("chars"), chars,
                    service.t("updated at"), note.updated_at()
                ));
            }

            // 右侧：扩展区域
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Rust Notes v0.1");
            });
        });
    }
}