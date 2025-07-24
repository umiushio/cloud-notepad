use tokio_util::sync::CancellationToken;

use crate::data::Note;
use crate::message::sync::SyncMessage;
use crate::message::Message;
use crate::message::sync::NoteResponse;
use crate::message::sync::SyncData;
use crate::message::sync::SyncResponse;
use super::AppState;
use super::NoteState;

pub trait SyncState {
    fn handle_response(&mut self, response: SyncResponse);
    fn cloud_update_note(&self, note: &Note) -> anyhow::Result<()>;
    fn cloud_import_note(&self, note: &Note) -> anyhow::Result<()>;
    fn start_periodic_sync(&mut self);
    fn last_sync_time(&self) -> Option<String>;
}

impl SyncState for AppState {
    fn handle_response(&mut self, response: SyncResponse) {
        if let Some(sync_data) = response.sync_data() {
            match sync_data {
                SyncData::NoteId(note_id) => {
                    if let Err(e) = self.mark_sync(&note_id) {
                        println!("mark sync note {} failed: {}", note_id, e);
                    }
                }
                SyncData::Note(note) => {
                    let note_id = note.id().to_string();
                    if let Err(e) = self.sync_note(note) {
                        println!("sync note {} failed: {}", &note_id, e);
                    }
                }
                SyncData::NoteResponse(note_response) => {
                    if let Err(e) = self.apply_sync(note_response) {
                        println!("apply sync failed: {}", e);
                    }
                }
            }
        }
    }

    fn cloud_update_note(&self, note: &Note) -> anyhow::Result<()> {
        if self.user_name.is_some() {
            let sender = self.sender.clone();
            if let Err(e) = sender.try_send(Message::SyncMessage(SyncMessage::UpdateNote { 
                id: note.id().to_string(), 
                title: Some(note.title().to_string()), 
                content: Some(note.content().to_string()), 
                tags: Some(note.tags().clone()), 
                updated_at: note.updated_at(), 
            })) {
                eprintln!("send update message failed: {}", e);
            }
        }

        Ok(())
    }

    fn cloud_import_note(&self, note: &Note) -> anyhow::Result<()> {
        if self.user_name.is_some() {
            let sender = self.sender.clone();
            if let Err(e) = sender.try_send(Message::SyncMessage(SyncMessage::ImportNote {
                id: note.id().to_string(),
                title: note.title().to_string(),
                content: note.content().to_string(),
                tags: note.tags().clone(),
                created_at: note.created_at(),
                updated_at: note.updated_at(),
            })) {
                eprintln!("send import message failed: {}", e);
            }
        }

        Ok(())
    }

    fn start_periodic_sync(&mut self) {
        println!("start periodic sync");
        let sender = self.sender.clone();
        let sync_interval = tokio::time::Duration::from_secs(180); // 每3min同步一次
        let last_sync_time = self.last_sync_time.clone();
        let cancel_token = CancellationToken::new();
        self.sync_cancel_token = Some(cancel_token.clone());

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(sync_interval);
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        let last_sync_time = *last_sync_time.lock().unwrap();
                        if let Err(e) = sender.try_send(crate::message::Message::SyncMessage(
                            crate::message::sync::SyncMessage::SyncNotes { last_sync_time }
                        )) {
                            eprintln!("send sync notes message failed: {}", e);
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        println!("Periodic sync cancelled");
                        break;
                    }
                }
            }
        });
    }

    fn last_sync_time(&self) -> Option<String> {
        let last_sync_time = self.last_sync_time.lock().unwrap();
        last_sync_time.map(|time| format!("{}", time.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M")))
    }
}

impl AppState {
    fn apply_sync(&mut self, resp: NoteResponse) -> anyhow::Result<()>{
        let mut err_vec = Vec::new();
        for note in resp.notes {
            let note_id = note.id().to_string();
            if let Err(e) = self.sync_note(note) {
                err_vec.push(format!("Failed to sync note. Id: {}, error: {}.", &note_id, e));
            }
        }
        for deleted_note_id in resp.deleted_note_ids.iter() {
            if let Err(e) = self.delete_note(deleted_note_id) {
                err_vec.push(format!("Failed to delete note. Id: {}, error: {}.", deleted_note_id, e));
            }
        }
        if err_vec.is_empty() {
            let mut last_sync_time = self.last_sync_time.lock().unwrap();
            *last_sync_time = Some(resp.current_time);
            Ok(())
        } else {
            Err(anyhow::anyhow!(err_vec.join("\n")))
        }
    }

    fn mark_sync(&self, note_id: &str) -> anyhow::Result<()> {
        let mut conn = self.db_conn.lock().unwrap();
        conn.sync_note(note_id)?;
        Ok(())
    }

    // 同步笔记，以最后修改时间较新的为准
    fn sync_note(&mut self, note: Note) -> anyhow::Result<()> {
        let note_id = note.id().to_string();
        let local_updated_time = self.get_note(&note_id).map(|note| note.updated_at());
        if local_updated_time.is_none_or(|t| t < note.updated_at()) {
            self.rewrite_note(note)?;
            self.mark_sync(&note_id)?;
        }
        
        Ok(())
    }
}