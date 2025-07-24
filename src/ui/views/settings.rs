use crate::{i18n::{Language, Translate}, io::{ExportFormat, MergeStrategy}, state::{SettingsState, Theme}};

pub struct SettingsView {
    is_open: bool,
    pos: egui::Pos2,
    show_submenu: Option<ShowSubmenu>,
    
}

impl SettingsView {
    pub fn new() -> Self {
        Self {
            is_open: false,
            pos: egui::Pos2::default(),
            show_submenu: None,
        }
    }

    pub fn show<T: SettingsState + Translate>(&mut self, ui: &mut egui::Ui, state: &mut T) {
        if !self.is_open {
            self.show_submenu = None;
            return
        }

        let width = 80.0;
        let mut should_close = true;

        let menu_response = egui::Window::new("settings")
            .title_bar(false)
            .resizable(false)
            .fixed_pos(self.pos)
            .fixed_size(egui::vec2(width, 0.0))
            .show(ui.ctx(), |ui| {
                // ui.set_width(80.0);

                let submenus = vec![
                    ShowSubmenu::Theme,
                    ShowSubmenu::Language,
                    ShowSubmenu::None,
                    ShowSubmenu::Export,
                    ShowSubmenu::Import,
                ];

                for submenu in submenus.iter() {
                    match submenu {
                        ShowSubmenu::None => {
                            ui.separator();
                        }
                        submenu => {
                            if ui.button(submenu.title().unwrap()).clicked() {
                                let submenu = Some(submenu.to_owned());
                                self.show_submenu = if submenu == self.show_submenu {
                                    None
                                } else {
                                    submenu
                                }
                            }
                        }
                    }
                }
            });

        should_close &= menu_response
            .map(|r| r.response.clicked_elsewhere())
            .unwrap_or(true);

        if let Some(show_submenu) = self.show_submenu.clone().as_ref() {
            let pos = self.pos + egui::vec2(width + 15.0, 0.0);
            let submenu_ctx = show_submenu.ctx(pos).unwrap();
            should_close &= self.show_submenu(ui, &submenu_ctx, |ui| show_submenu.add_contents(ui, state));
        }

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) || should_close {
            self.is_open = false;
        }
    }

    fn show_submenu(&self, ui: &mut egui::Ui, submenu_ctx: &SubmenuContext, add_contents: impl FnOnce(&mut egui::Ui)) -> bool {
        let submenu_response = egui::Window::new(&submenu_ctx.id)
            .title_bar(false)
            .resizable(false)
            .fixed_pos(submenu_ctx.pos)
            .fixed_size(egui::vec2(submenu_ctx.width, 0.0))
            .show(ui.ctx(), |ui| {
                add_contents(ui);
            });

        submenu_response
            .map(|r| r.response.clicked_elsewhere())
            .unwrap_or(true)
    }

    pub fn open(&mut self, pos: egui::Pos2) {
        self.is_open = true;
        self.pos = pos;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ShowSubmenu {
    Theme,
    Language,
    Export,
    Import,
    None,       // 分隔符
}

impl ShowSubmenu {
    fn title(&self) -> Option<&'static str> {
        match self {
            ShowSubmenu::Theme => Some("Theme"),
            ShowSubmenu::Language => Some("Language"),
            ShowSubmenu::Export => Some("Export"),
            ShowSubmenu::Import => Some("Import"),
            _ => None,
        }
    }

    fn ctx(&self, pos: egui::Pos2) -> Option<SubmenuContext> {
        match self {
            ShowSubmenu::Theme => Some(SubmenuContext {
                width: 100.0,
                pos,
                id: "theme_submenu".to_string(),
            }),
            ShowSubmenu::Language => Some(SubmenuContext {
                width: 100.0,
                pos: pos + egui::vec2(0.0, 25.0),
                id: "language_submenu".to_string(),
            }),
            ShowSubmenu::Export => Some(SubmenuContext {
                width: 200.0,
                pos: pos + egui::vec2(0.0, 12.0),
                id: "export_submenu".to_string(),
            }),
            ShowSubmenu::Import => Some(SubmenuContext {
                width: 200.0,
                pos: pos + egui::vec2(0.0, 6.0),
                id: "import_submenu".to_string(),
            }),
            _ => None,
        }
    }

    fn add_contents<T: SettingsState>(&self, ui: &mut egui::Ui, t: &mut T) {
        match self {
            ShowSubmenu::Theme => {
                ui.radio_value(t.theme_mut(), Theme::Dark, "Dark");
                ui.radio_value(t.theme_mut(), Theme::Light, "Light");
            }
            ShowSubmenu::Language => {
                ui.radio_value(t.language_mut(), Language::English, "English");
                ui.radio_value(t.language_mut(), Language::Chinese, "Chinese");
                ui.radio_value(t.language_mut(), Language::Japanese, "Janpanse");
            }
            ShowSubmenu::Export => {
                ui.radio_value(&mut t.export_config_mut().format, ExportFormat::Markdown(true), "Markdown(Include Metadata)");
                ui.radio_value(&mut t.export_config_mut().format, ExportFormat::Markdown(false), "Markdown(Exclude Metadata)");
                ui.radio_value(&mut t.export_config_mut().format, ExportFormat::Json, "Json");
                ui.radio_value(&mut t.export_config_mut().format, ExportFormat::Html, "Html");
            }
            ShowSubmenu::Import => {
                ui.radio_value(&mut t.import_config_mut().merge_strategy, MergeStrategy::Skip, "Skip");
                ui.radio_value(&mut t.import_config_mut().merge_strategy, MergeStrategy::Rename, "Rename");
                ui.radio_value(&mut t.import_config_mut().merge_strategy, MergeStrategy::Overwrite, "Overwrite");
                ui.separator();
                ui.checkbox(&mut t.import_config_mut().preserve_timestamps, "Preserve Timestamps");
            }
            _ => ()
        }
    }
}

struct SubmenuContext {
    width: f32,
    pos: egui::Pos2,
    id: String,
}