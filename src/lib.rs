pub mod service;
pub use service::state::AppState;

pub mod data;
pub use data::db::Database;
pub use data::model::{Note, Notebook};

pub mod ui;
pub use ui::app::NoteApp;
