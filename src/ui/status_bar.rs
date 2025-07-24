use crate::{i18n::Translate, state::{AuthState, SyncState, TabState}};

#[derive(Default)]
pub struct StatusBar {

}

impl StatusBar {
    pub fn show<T: AuthState + TabState + SyncState + Translate>(&self, ui: &mut egui::Ui, state: &T) {
        ui.horizontal(|ui| {
            // 左侧：账户信息
            // ...
            if let Some(user_name) = state.user_name() {
                ui.label(&format!("✔ {}", user_name));
            } else {
                ui.label(&format!("✘ {}", state.t("guest")));
            }

            // 中间: 笔记信息
            if let Some(note) = state.current_note() {
                let words = note.content().split_whitespace().count();
                let chars = note.content().chars().count();

                ui.separator();
                ui.label(format!(
                    "📄 {} | {}: {} | {}: {} | {}: {}",
                    note.title(),
                    state.t("words"), words,
                    state.t("chars"), chars,
                    state.t("updated at"), note.updated_at_as_str()
                ));
            }

            // 右侧：扩展区域
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Rust Notes v0.1");

                // 最后同步时间
                if let Some(last_sync_time) = state.last_sync_time() {
                    ui.separator();
                    ui.label(&format!(
                        "{}: {}",
                        state.t("last sync time"),
                        last_sync_time,
                    ));
                }
            });
        });
    }
}