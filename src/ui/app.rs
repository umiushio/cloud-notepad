use std::sync::Arc;
use egui::{FontData, FontDefinitions, FontFamily};
use tokio::sync::{Mutex, mpsc};
use crate::message::{Message, Response};
use crate::logger::{ClientLogger, MemoryLogger};
use crate::AppState;
use crate::state::{AuthState, NoteState, SettingsState, SyncState, Theme};
use super::app_layout::AppLayout;

pub struct NoteApp {
    state: Arc<Mutex<AppState>>,
    layout: AppLayout,
    response_receiver: mpsc::Receiver<Response>,
    _logger: Arc<ClientLogger>,
}

impl NoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>, message_sender: mpsc::Sender<Message>, response_receiver: mpsc::Receiver<Response>) -> anyhow::Result<Self> {
        // 初始化日志系统
        let memory_logger = MemoryLogger::new(1000);
        let _logger = ClientLogger::new()
            .enable_console(tracing::Level::DEBUG)
            .enable_file("logs", tracing::Level::INFO)
            .init(Some(memory_logger.clone()));
            
        
        let state = AppState::new(message_sender, memory_logger)?;

        // 设置初始主题
        match state.theme() {
            Theme::Dark => cc.egui_ctx.set_visuals(egui::Visuals::dark()),
            _ => ()
        }

        // 设置字体
        Self::setup_fonts(&cc.egui_ctx);

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            layout: AppLayout::new(),
            response_receiver,
            _logger,
        })
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        
        // 添加常规字体
        fonts.font_data.insert(
            "source_han_sans_regular".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../../assets/fonts/SourceHanSans-Regular.ttc"))),
        );

        // 添加粗体字体
        fonts.font_data.insert(
            "source_han_sans_bold".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../../assets/fonts/SourceHanSans-Bold.ttc"))),
        );

        // 修改字体族配置
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "source_han_sans_regular".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .extend(vec!["source_han_sans_regular".to_owned()]);

        fonts
            .families
            .entry(FontFamily::Name("Bold".into()))
            .or_default()
            .insert(0, "source_han_sans_bold".to_owned());

        ctx.set_fonts(fonts);
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 防抖批量保存
        if let Ok(mut state) = self.state.try_lock() {
            if state.debounce_last_edit.map_or(false, |t| t.elapsed() > std::time::Duration::from_secs(5)) {
                let note_ids: Vec<_> = state.debounce_modified.drain().collect();
                for note_id in note_ids.iter() {
                    if let Some(note) = state.get_note(note_id) {
                        // 1. 保存到本地
                        let _ = state.save_note(&note);
                        // 2. 云同步
                        let _ = state.cloud_update_note(&note);
                    }
                }
            }
        }
        
        // 处理来自服务的响应
        while let Ok(response) = self.response_receiver.try_recv() {
            while let Ok(mut state) = self.state.try_lock() {
                match response {
                    Response::AuthResponse(response) => {
                        if response.success() {
                            state.update_auth(response);
                        } else {
                            eprintln!("auth error: {}", response.error().unwrap_or("unknown error"));
                        }
                    }
                    Response::SyncResponse(response) => {
                        if response.success() {
                            state.handle_response(response);
                        } else {
                            eprintln!("sync error: {}", response.error().unwrap_or("unknown error"));
                        }
                    }
                }
                break;
            }
        }
        
        if let Ok(mut state) = self.state.try_lock() {
            self.layout.show(ctx, &mut state);
        }
    }

}