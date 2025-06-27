use chrono::{DateTime, Local, Utc};

#[derive(Debug)]
pub struct DeleteNote {
    pub(in crate::data) id: String,
    pub(in crate::data) title: String,
    pub(in crate::data) deleted_at: DateTime<Utc>,
}

impl DeleteNote {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn deleted_at(&self) -> String {
        format!("{}", self.deleted_at.with_timezone(&Local).format("%Y-%m-%d %H:%M"))
    }
}