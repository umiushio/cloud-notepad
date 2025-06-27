use std::sync::Arc;
use egui::{FontData, FontDefinitions, FontFamily};
use crate::AppState;
use crate::services::{SettingsService, Theme};
use super::app_layout::AppLayout;

pub struct NoteApp {
    state: AppState,
    layout: AppLayout,
}

impl NoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> anyhow::Result<Self> {
        let state = AppState::new()?;

        // 设置初始主题
        match state.theme() {
            Theme::Dark => cc.egui_ctx.set_visuals(egui::Visuals::dark()),
            _ => ()
        }

        // 设置字体
        Self::setup_fonts(&cc.egui_ctx);

        Ok(Self {
            state,
            layout: AppLayout::new(),
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
        self.layout.show(ctx, &mut self.state);

        // // 自动保存检查
        // if self.state.current_note_is_modified() {
        //     if let Err(e) = self.state.save_current_note() {
        //         eprintln!("自动保存失败: {}", e);
        //     }
        // }
    }

    // fn on_close_event(&mut self) -> bool {
    //     if self.state.needs_save {
    //         if let Err(e) = self.state.save_changes() {
    //             eprintln!("退出前保存失败: {}", e);
    //         }
    //     }
    //     true
    // }
}