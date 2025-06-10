pub mod app;
pub mod main_window;
pub mod editor;
pub mod markdown_editor;
pub mod sidebar;
pub mod status_bar;

use crate::Note;
use crate::AppState;
use crate::i18n::Translate;

use main_window::MainWindow;
use editor::NoteEditor;
use sidebar::NoteSidebar;
use markdown_editor::MarkdownEditor;
use status_bar::StatusBar;