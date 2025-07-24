pub mod auth;
pub mod sync;

#[derive(Debug)]
pub enum Message {
    AuthMessage(auth::AuthMessage),
    SyncMessage(sync::SyncMessage),
}

#[derive(Debug)]
pub enum Response {
    AuthResponse(auth::AuthResponse),
    SyncResponse(sync::SyncResponse),
}