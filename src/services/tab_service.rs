use crate::data::Note;
use super::AppState;

pub trait TabService {
    fn recent_notes(&self) -> Vec<&String>;
    fn current_note_id(&self) -> Option<&String>;
    fn current_note(&self) -> Option<Note>;
    fn current_note_id_equals(&self, id: &str) -> bool;
    fn load_note(&mut self, note_id: &str);
    fn close_note(&mut self, note_id: &str);
}

impl TabService for AppState {
    fn recent_notes(&self) -> Vec<&String> {
        self.recent_notes.get_visible_tabs()
    }

    fn current_note_id(&self) -> Option<&String> {
        self.recent_notes.current_tab()
    }

    fn current_note(&self) -> Option<Note> {
        let notebook = self.notebook.lock().unwrap();
        self.current_note_id()
            .and_then(|id| notebook.find_note(id))
    }

    fn current_note_id_equals(&self, id: &str) -> bool {
        self.current_note_id().cloned() == Some(id.to_string())
    }

    fn load_note(&mut self, note_id: &str) {
        self.recent_notes.add_or_activate(note_id.to_string());
    }

    fn close_note(&mut self, note_id: &str) {
        self.recent_notes.remove(&note_id.to_string());
    }
}
