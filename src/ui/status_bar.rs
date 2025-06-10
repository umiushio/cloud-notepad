use super::*;

#[derive(Default)]
pub struct StatusBar {

}

impl StatusBar {
    pub fn show<T: Translate>(&self, ui: &mut egui::Ui, words: usize, last_updated: String, t: &T) {
        egui::TopBottomPanel::bottom("status_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{}: {}", t.t("words"), words));
                ui.separator();
                ui.label(format!("{}: {}", t.t("updated"), last_updated));
            });
        });
    }
}