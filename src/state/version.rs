use crate::data::{Note, NoteVersion};
use super::AppState;
use super::NoteState;

pub trait VersionState {
    fn list_versions(&self, note_id: &str) -> anyhow::Result<Vec<NoteVersion>>;
    fn save_version(&mut self, comment: &str, note: &Note) -> anyhow::Result<()>;
    fn restore_version(&mut self, note_version: &NoteVersion) -> anyhow::Result<()>;
    fn delete_version(&mut self, version_id: &str) -> anyhow::Result<()>;
}

impl VersionState for AppState {
    fn list_versions(&self, note_id: &str) -> anyhow::Result<Vec<NoteVersion>> {
        let conn = self.db_conn.lock().unwrap();
        let note_versions = conn.load_version_history(note_id)?;
        Ok(note_versions)
    }

    fn save_version(&mut self, comment: &str, note: &Note) -> anyhow::Result<()> {
        let note_version = NoteVersion::new(comment, note);
        let mut conn = self.db_conn.lock().unwrap();
        conn.save_version(&note_version)?;
        Ok(())
    }

    fn restore_version(&mut self, note_version: &NoteVersion) -> anyhow::Result<()> {
        if let Some(mut note) = self.get_note(note_version.note_id()) {
            note.updated_by_note_version(note_version);
            self.update_note(note)?;
        }
        Ok(())
    }

    fn delete_version(&mut self, version_id: &str) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.delete_version(version_id)?;
        Ok(())
    }
}