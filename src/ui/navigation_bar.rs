use crate::{i18n::Translate, state::{AuthState, SettingsState}, ui::views::{AccountView, SettingsView}};

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
    account_view: AccountView,
    settings_view: SettingsView,
}

impl NavigationBar {
    pub fn new() -> Self {
        Self {
            selected: NavigationTab::Notes,
            account_view: AccountView::new(),
            settings_view: SettingsView::new(),
        }
    }

    pub fn selected(&self) -> NavigationTab {
        self.selected
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, t: &mut T) 
        where T: AuthState + SettingsState + Translate {
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

            // 账户菜单
            let account_response = ui.add(
                egui::Button::new(NavigationTab::Account.icon())
                    .frame(false)
                    .min_size(egui::vec2(32.0, 32.0))
            )
            .on_hover_text(&t.t(NavigationTab::Account.tooltip()));
            // 账户菜单窗口
            self.account_view.show(ui, t);

            if account_response.clicked() {
                self.account_view.open(account_response.rect.right_top() + egui::vec2(12.0, -24.0));
            }


            // 设置菜单
            let settings_response = ui.add(
                egui::Button::new(NavigationTab::Settings.icon())
                    .frame(false)
                    .min_size(egui::vec2(32.0, 32.0))
            )
            .on_hover_text(&t.t(NavigationTab::Settings.tooltip()));
            // 设置菜单窗口
            self.settings_view.show(ui, t);

            if settings_response.clicked() {
                self.settings_view.open(settings_response.rect.right_top() + egui::vec2(12.0, -64.0));
            }
        });
    }
}