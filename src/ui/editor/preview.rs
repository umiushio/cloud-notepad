use crate::markdown::renderer::MarkdownRenderer;

/// Markdown编辑器状态
#[derive(Default)]
pub struct MarkdownPreview {
    
}

impl MarkdownPreview {
    /// 显示编辑器
    pub fn show(&mut self, ui: &mut egui::Ui, text: &str, cursor_pos: Option<usize>) {
        // 预览区 (只读), 使用 Markdown 渲染
        let mut render = MarkdownRenderer::new(text);
        let layout_job = render.render(cursor_pos);
        ui.label(layout_job);

        // // 渲染和编辑同窗口
        // let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
        //     if self.show_rendering {
        //         // 使用 Markdown 渲染
        //         let mut render = MarkdownRenderer::new(text);
        //         let mut layout_job = render.render(text, self.cursor_pos);
        //         layout_job.wrap.max_width = wrap_width;
        //         ui.fonts(|f| f.layout_job(layout_job))
        //     } else {
        //         // 纯文本模式
        //         ui.fonts(|f| f.layout_no_wrap(text.to_string(), FontId::monospace(14.0), Color32::WHITE))
        //     }
        // };
        
        // let response = egui::TextEdit::multiline(text)
        //     .desired_width(f32::INFINITY)
        //     .layouter(&mut layouter)
        //     .show(ui);
    }
}