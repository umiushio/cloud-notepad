mod notes_view;
mod tags_view;
mod search_view;
mod trash_view;

use crate::AppState;
use super::navigation_bar::NavigationTab;
use notes_view::NotesView;
use tags_view::TagsView;
use search_view::SearchView;
use trash_view::TrashView;

pub struct Sidebar {
    notes_view: NotesView,
    tags_view: TagsView,
    search_view: SearchView,
    trash_view: TrashView,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            notes_view: NotesView::default(),
            tags_view: TagsView::default(),
            search_view: SearchView::default(),
            trash_view: TrashView::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut AppState, tab: NavigationTab) {
        // 侧边栏内容
        match tab {
            NavigationTab::Notes => self.notes_view.show(ui, state),
            NavigationTab::Tags => self.tags_view.show(ui, state),
            NavigationTab::Search => self.search_view.show(ui, state),
            NavigationTab::Trash => self.trash_view.show(ui, state),
            _ => (),
        }
    }
}