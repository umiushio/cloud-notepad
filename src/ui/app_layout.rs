use super::{menu_bar::MenuBar, navigation_bar::NavigationBar, sidebar::Sidebar, editor::EditorPanel, status_bar::StatusBar, views::{VersionHistoryView, LogView}};
use crate::{state::log::LogState, AppState};
use crate::state::TabState;

pub struct AppLayout {
    menu_bar: MenuBar,
    navigation_bar: NavigationBar,
    sidebar: Sidebar,
    editor: EditorPanel,
    status_bar: StatusBar,

    show_view: ShowView,
    version_history: VersionHistoryView,
    logs: LogView,
}

impl AppLayout {
    pub fn new() -> Self {
        Self {
            menu_bar: MenuBar::default(),
            navigation_bar: NavigationBar::new(),
            sidebar: Sidebar::new(),
            editor: EditorPanel::default(),
            status_bar: StatusBar::default(),
            show_view: ShowView::default(),
            version_history: VersionHistoryView::default(),
            logs: LogView::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut AppState) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if let Some(show_view) = self.menu_bar.show(ui, state) {
                self.show_view = show_view;
            }
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.status_bar.show(ui, state);
        });

        // 左侧导航栏
        egui::SidePanel::left("navigation_bar")
            .resizable(false)
            .default_width(40.0)
            .show(ctx, |ui| {
                self.navigation_bar.show(ui, state);
            });
        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            // 侧边栏
            egui::SidePanel::left("sidebar_content")
                .resizable(true)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    self.sidebar.show(ui, state, self.navigation_bar.selected());
                });
            // 编辑区域
            self.editor.show(ui, state);
        });

        // 显示版本历史窗口
        if self.show_view.show_version_history {
            self.version_history.open(state.current_note_id().cloned());
            if !self.version_history.show(ctx, state) {
                self.show_view.show_version_history = false;
            }
        }
        // 显示日志窗口
        if self.show_view.show_logs {
            self.logs.update_logs(state.get_logs());
            if !self.logs.show(ctx) {
                self.show_view.show_logs = false;
            }
        }

    }
}

// 显示视图
#[derive(Default)]
pub struct ShowView {
    pub show_version_history: bool,
    pub show_logs: bool,
}