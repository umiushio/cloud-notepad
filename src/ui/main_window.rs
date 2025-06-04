use super::*;

#[derive(Default)]
pub struct MainWindow {
    editor: NoteEditor,
    sidebar: NoteSidebar,
}

impl MainWindow {
    pub fn show(&mut self, ctx: &egui::Context, state: &mut AppState) {
        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                self.sidebar.show(ui, state);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(mut note) = state.current_note() {
                self.editor.show(ui, &mut note);
                state.update_note(note).unwrap();
            } else {
                ui.label("Please select a note from the sidebar or create a new note");
            }
        });
    }
}