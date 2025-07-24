use std::sync::atomic::Ordering;
use crate::{message::{auth::{AuthMessage, AuthResponse}, Message}, state::{NoteState, SyncState}};
use super::AppState;

pub trait AuthState {
    fn is_authenticated(&self) -> bool;
    fn update_auth(&mut self, response: AuthResponse);
    fn register(&mut self, name: &str, email: &str, password: &str);
    fn login(&mut self, email: &str, password: &str);
    fn logout(&mut self);
    fn user_name(&self) -> Option<&str>;
}

impl AuthState for AppState {
    fn is_authenticated(&self) -> bool {
        self.is_authenticated.load(Ordering::Relaxed)
    }

    fn update_auth(&mut self, response: AuthResponse) {
        self.is_authenticated.store(response.success(), Ordering::Relaxed);
        if let Some(err) = response.error() {
            println!("Auth error: {}", err);
        } else {
            self.user_name = response.user_name();
            self.user_id = response.user_id().to_string();
            if let Err(e) = self.reload_notes(&self.user_id) {
                eprintln!("Failed to reload notebook: {}", e);
            }
            if self.user_name.is_some() {
                // 登录成功后启动定时同步
                self.start_periodic_sync();
            } else {
                // 否则取消同步线程
                if let Some(token) = self.sync_cancel_token.take() {
                    token.cancel();
                }
                // 清空最后同步时间
                *self.last_sync_time.lock().unwrap() = None;
            }
        }
    }

    fn register(&mut self, name: &str, email: &str, password: &str) {
        self.is_authenticated.store(false, Ordering::Relaxed);
        let sender = self.sender.clone();
        if let Err(e) = sender.try_send(Message::AuthMessage(AuthMessage::Register { 
            name: name.to_string(),
            email: email.to_string(), 
            password: password.to_string() 
        })) {
            eprintln!("send register message failed: {}", e);
        }
    }

    fn login(&mut self, email: &str, password: &str) {
        self.is_authenticated.store(false, Ordering::Relaxed);
        let sender = self.sender.clone();
        if let Err(e) = sender.try_send(Message::AuthMessage(AuthMessage::Login { 
            email: email.to_string(), 
            password: password.to_string() 
        })) {
            eprintln!("send login message failed: {}", e);
        }
    }

    fn logout(&mut self) {
        self.is_authenticated.store(false, Ordering::Relaxed);
        let sender = self.sender.clone();
        if let Err(e) = sender.try_send(Message::AuthMessage(AuthMessage::Logout)) {
            eprintln!("send register message failed: {}", e);
        }
    }

    fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

// pub trait AuthState {
//     fn register<'a>(&'a mut self, email: &'a str, password: &'a str) -> AuthAsyncType<'a, ()>;
//     fn login<'a>(&'a mut self, email: &'a str, password: &'a str) -> AuthAsyncType<'a, ()>;
//     fn logout(&mut self) -> AuthAsyncType<'_, ()>;

//     fn validate_password(password: &str) -> bool;
//     fn user_id(&self) -> Option<&str>;
// }

// impl AuthState for AppState {
//     fn register<'a>(&'a mut self, email: &'a str, password: &'a str) -> AuthAsyncType<'a, ()> {
//         println!("Begin Register... email: {}", email);
//         let client = self.client.clone();
//         Box::pin(async move {
//             let mut client = client.lock().await;
//             self.user_id = client.register(email, password).await?;
//             println!("user id: {:?}", self.user_id);
//             Ok(())
//         })
//     }

//     fn login<'a>(&'a mut self, email: &'a str, password: &'a str) -> AuthAsyncType<'a, ()> {
//         let client = self.client.clone();
//         Box::pin(async move {
//             let mut client = client.lock().await;
//             self.user_id = client.login(email, password).await?;
//             Ok(())
//         })
//     }

//     fn logout(&mut self) -> AuthAsyncType<'_, ()> {
//         let client = self.client.clone();
//         Box::pin(async move {
//             let client = client.lock().await;
//             client.logout().await?;
//             self.user_id = None;
//             Ok(())
//         })
//     }

//     fn validate_password(password: &str) -> bool {
//         let n = password.len();
//         n >= 6 && n <= 16
//     }

//     fn user_id(&self) -> Option<&str> {
//         self.user_id.as_deref()
//     }
// }

