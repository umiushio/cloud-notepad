use crate::{i18n::{Language, Translate}, io::{ExportFormat, MergeStrategy}, services::{SettingsService, Theme}};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NavigationTab {
    Notes,
    Tags,
    Search,
    Trash,
    Account,
    Settings,
}

impl NavigationTab {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Notes => "📝",
            Self::Tags => "🏷️",
            Self::Search => "🔍",
            Self::Trash => "🗑️",
            Self::Account => "👤",
            Self::Settings => "⚙️",
        }
    }

    pub fn tooltip(&self) -> &'static str {
        match self {
            Self::Notes => "notes",
            Self::Tags => "tags",
            Self::Search => "search",
            Self::Trash => "trash",
            Self::Account => "account",
            Self::Settings => "settings",
        }
    }
}

pub struct NavigationBar {
    selected: NavigationTab,
    show_menu: Option<ShowMenu>,
    show_submenu: Option<ShowSubmenu>,
}

impl NavigationBar {
    pub fn new() -> Self {
        Self {
            selected: NavigationTab::Notes,
            show_menu: None,
            show_submenu: None,
        }
    }

    pub fn selected(&self) -> NavigationTab {
        self.selected
    }

    pub fn show<T: SettingsService + Translate>(&mut self, ui: &mut egui::Ui, t: &mut T) {
        ui.vertical_centered(|ui| {
            // 主功能Tabs
            for tab in &[NavigationTab::Notes, NavigationTab::Tags, NavigationTab::Search, NavigationTab::Trash] {
                let response = ui.add(
                    egui::Button::new(tab.icon())
                        .frame(false)
                        .min_size(egui::vec2(40.0, 40.0))
                )
                .on_hover_text(&t.t(tab.tooltip()));

                if response.clicked() {
                    self.selected = *tab;
                }

                // 高亮选中状态
                if self.selected == *tab {
                    response.highlight();
                }
            }

            // 底部Tabs
            ui.add_space(ui.available_height() - 80.0);

            let mut should_close = true;
            // 账户菜单
            let account_response = ui.add(
                egui::Button::new(NavigationTab::Account.icon())
                    .frame(false)
                    .min_size(egui::vec2(32.0, 32.0))
            )
            .on_hover_text(&t.t(NavigationTab::Account.tooltip()));

            if account_response.clicked() {
                self.show_menu = self.show_menu.clone().xor(Some(ShowMenu::Account));
                should_close = false;
            }


            // 设置菜单
            let settings_response = ui.add(
                egui::Button::new(NavigationTab::Settings.icon())
                    .frame(false)
                    .min_size(egui::vec2(32.0, 32.0))
            )
            .on_hover_text(&t.t(NavigationTab::Settings.tooltip()));

            if settings_response.clicked() {
                self.show_menu = self.show_menu.clone().xor(Some(ShowMenu::Settings));
                should_close = false;
            }

            // 菜单窗口
            if let Some(show_menu) = self.show_menu.as_ref() {
                // 显示主菜单窗口
                let pos = match show_menu {
                    ShowMenu::Account => account_response.rect.right_top() + egui::vec2(10.0, -40.0),
                    ShowMenu::Settings => settings_response.rect.right_top() + egui::vec2(10.0, -64.0),
                };
                let menu_ctx = show_menu.ctx(pos);
                should_close &= self.show_menu(ui.ctx(), &menu_ctx);

                // 显示子菜单窗口
                if let Some(show_submenu) = self.show_submenu.as_ref() {
                    let pos = pos + egui::vec2(menu_ctx.width + 12.0, 0.0);
                    let submenu_ctx = show_submenu.ctx(pos).unwrap();
                    should_close &= self.show_submenu(ui.ctx(), &submenu_ctx, |ui| show_submenu.add_contents(ui, t));
                }

                // 判断是否要关闭菜单
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) || should_close {
                    self.show_menu = None;
                    self.show_submenu = None;
                }
            }
        });
    }

    fn show_menu(&mut self, ctx: &egui::Context, menu_ctx: &MenuContext) -> bool {
        let menu_response = egui::Window::new(&menu_ctx.id)
            .title_bar(false)
            .resizable(false)
            .fixed_pos(menu_ctx.pos)
            .fixed_size(egui::vec2(menu_ctx.width, 0.0))
            .show(ctx, |ui| {
                ui.set_width(menu_ctx.width);

                for submenu in menu_ctx.submenus.iter() {
                    match submenu {
                        ShowSubmenu::None => {
                            ui.separator();
                        }
                        submenu => {
                            if ui.button(submenu.title().unwrap()).clicked() {
                                let submenu = Some(submenu.clone());
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

        menu_response
            .map(|r| r.response.clicked_elsewhere())
            .unwrap_or(true)
    }

    fn show_submenu(
        &self, 
        ctx: &egui::Context, 
        submenu_ctx: &SubmenuContext,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> bool {
        let submenu_response = egui::Window::new(&submenu_ctx.id)
            .title_bar(false)
            .resizable(false)
            .fixed_pos(submenu_ctx.pos)
            .fixed_size(egui::vec2(submenu_ctx.width, 0.0))
            .show(ctx, |ui| {
                ui.set_width(submenu_ctx.width);
                add_contents(ui);
            });

        submenu_response
            .map(|r| r.response.clicked_elsewhere())
            .unwrap_or(true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ShowMenu {
    Account,
    Settings,
}

impl ShowMenu {
    fn ctx(&self, pos: egui::Pos2) -> MenuContext {
        match self {
            ShowMenu::Account => MenuContext {
                width: 80.0,
                pos,
                id: "account_menu".to_string(),
                submenus: vec![],
            },
            ShowMenu::Settings => MenuContext {
                width: 80.0,
                pos,
                id: "settings_menu".to_string(),
                submenus: vec![
                    ShowSubmenu::Theme, 
                    ShowSubmenu::Language, 
                    ShowSubmenu::None,
                    ShowSubmenu::Export,
                    ShowSubmenu::Import,
                ],
            },
        }
    }
}

struct MenuContext {
    width: f32,
    pos: egui::Pos2,
    id: String,
    submenus: Vec<ShowSubmenu>,
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

    fn add_contents<T: SettingsService>(&self, ui: &mut egui::Ui, t: &mut T) {
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