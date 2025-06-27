use super::{menu_bar::MenuBar, navigation_bar::NavigationBar, sidebar::Sidebar, editor::EditorPanel, status_bar::StatusBar, version_history_view::VersionHistoryView};
use crate::AppState;
use crate::services::TabService;

pub struct AppLayout {
    menu_bar: MenuBar,
    navigation_bar: NavigationBar,
    sidebar: Sidebar,
    editor: EditorPanel,
    status_bar: StatusBar,

    show_view: Option<ShowView>,
    version_history: VersionHistoryView,
}

impl AppLayout {
    pub fn new() -> Self {
        Self {
            menu_bar: MenuBar::default(),
            navigation_bar: NavigationBar::new(),
            sidebar: Sidebar::new(),
            editor: EditorPanel::default(),
            status_bar: StatusBar::default(),
            show_view: None,
            version_history: VersionHistoryView::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut AppState) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if let Some(view) = self.menu_bar.show(ui, state) {
                self.show_view = Some(view);
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
        if let Some(show_view) = &self.show_view {
            match show_view {
                ShowView::ShowVersionHistory => {
                    self.version_history.open(state.current_note_id().cloned());
                    if !self.version_history.show(ctx, state) {
                        self.show_view = None;
                    }
                }
            }
        }
    }
}

// 显示视图
pub enum ShowView {
    ShowVersionHistory,
    // 其他动作...
}