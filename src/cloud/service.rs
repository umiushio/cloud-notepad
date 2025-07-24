use tokio::sync::mpsc;

use crate::{message::{auth::{AuthMessage, AuthResponse}, sync::{SyncMessage, SyncResponse}, Message, Response}, logger::{client::with_logging, log_error, AsyncLogType}};

use super::{client::CloudClient, error::AuthError};

pub struct CloudService {
    client: CloudClient,
    sender: mpsc::Sender<Response>,
}

impl CloudService {
    pub fn new(base_url: &str, sender: mpsc::Sender<Response>) -> Result<Self, AuthError> {
        Ok(Self {
            client: CloudClient::new(base_url)?,
            sender,
        })
    }

    pub async fn run(mut self, mut receiver: mpsc::Receiver<Message>) {
        while let Some(msg) = receiver.recv().await {
            match msg {
                Message::AuthMessage(msg) => {
                    match msg {
                        AuthMessage::Register { name, email, password } => {
                            let fut = self.client.register(&name, &email, &password);
                            let response = AuthResponse::from(
                                with_logging(
                                    AsyncLogType::Auth,
                                    "register",
                                    Some("email"),
                                    Some(&email),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::AuthResponse(response)).await {
                                log_error(&e.to_string(), "register send");
                            }
                        }
                        AuthMessage::Login { email, password } => {
                            let fut = self.client.login(&email, &password);
                            let response = AuthResponse::from(
                                with_logging(
                                    AsyncLogType::Auth,
                                    "login",
                                    Some("email"),
                                    Some(&email),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::AuthResponse(response)).await {
                                log_error(&e.to_string(), "login send");
                            }
                        }
                        AuthMessage::Logout => {
                            let fut = self.client.logout();
                            let response = AuthResponse::from(
                                with_logging(
                                    AsyncLogType::Auth,
                                    "logout",
                                    None,
                                    None,
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::AuthResponse(response)).await {
                                log_error(&e.to_string(), "logout send");
                            }
                        }
                    }
                }
                Message::SyncMessage(msg) => {
                    match msg {
                        SyncMessage::CreateNote { id, title, created_at} => {
                            let fut = self.client.create_note(&id, &title, created_at);
                            let response = SyncResponse::from(
                                with_logging(
                                    AsyncLogType::Sync,
                                    "create note",
                                    Some("note_id"),
                                    Some(&id),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::SyncResponse(response)).await {
                                log_error(&e.to_string(), "create note send");
                            }
                        }
                        SyncMessage::UpdateNote { id, title, content, tags, updated_at } => {
                            let fut = self.client.update_note(&id, title.as_deref(), content.as_deref(), tags, updated_at);
                            let response = SyncResponse::from(
                                with_logging(
                                    AsyncLogType::Sync,
                                    "update note",
                                    Some("note_id"),
                                    Some(&id),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::SyncResponse(response)).await {
                                log_error(&e.to_string(), "update note send");
                            }
                        }
                        SyncMessage::ImportNote { id, title, content, tags, created_at, updated_at } => {
                            let fut = self.client.import_note(&id, &title, &content, &tags, created_at, updated_at);
                            let response = SyncResponse::from(with_logging(
                                AsyncLogType::Sync,
                                "import note",
                                Some("note_id"),
                                Some(&id),
                                fut,
                            ).await);
                            if let Err(e) = self.sender.send(Response::SyncResponse(response)).await {
                                log_error(&e.to_string(), "import note send");
                            }
                        }
                        SyncMessage::DeleteNote { id } => {
                            let fut = self.client.delete_note(&id);
                            let response = SyncResponse::from(with_logging(
                                    AsyncLogType::Sync,
                                    "delete note",
                                    Some("note_id"),
                                    Some(&id),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::SyncResponse(response)).await {
                                log_error(&e.to_string(), "delete note send");
                            }
                        }
                        SyncMessage::SyncNotes { last_sync_time } => {
                            let fut = self.client.sync_notes(last_sync_time);
                            let last_sync_time_str = last_sync_time.map(|t| t.to_string());
                            let response = SyncResponse::from(with_logging(
                                    AsyncLogType::Sync,
                                    "sync notes",
                                    Some("last_sync_time"),
                                    last_sync_time_str.as_deref(),
                                    fut,
                                ).await
                            );
                            if let Err(e) = self.sender.send(Response::SyncResponse(response)).await {
                                log_error(&e.to_string(), "sync notes send");
                            }
                        }
                    }
                }
            }
        }
    }
}