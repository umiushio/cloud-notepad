mod nodes;
pub mod syntax;
pub mod renderer;

use nodes::Node;

/// 渲染上下文
struct RenderConfig<'a> {
    text: &'a str,
    cursor_pos: Option<usize>,
    theme: Theme,
}

impl<'a> RenderConfig<'a> {
    pub fn new(text: &'a str, cursor_pos: Option<usize>, theme: Theme) -> Self {
        Self { text, cursor_pos, theme }
    }

    pub fn belong_to(&self, range: &std::ops::Range<usize>) -> bool {
        if let Some(pos) = self.cursor_pos {
            range.contains(&pos)
        } else {
            false
        }
    }
}

/// 主题
#[derive(Clone)]
struct Theme {
    text_color: Color32,
    bold_color: Color32,
    code_color: Color32,
    code_bg_color: Color32,
    link_color: Color32,
    link_underline_color: Color32,
    underline_color: Color32,
    header_color: Color32,
    list_marker_color: Color32,
    task_pending_color: Color32,
    task_done_color: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text_color: Color32::from_gray(220),
            bold_color: Color32::from_rgb(255, 200, 100),
            code_color: Color32::from_rgb(150, 200, 150),
            code_bg_color: Color32::from_rgb(60, 60, 60),
            link_color: Color32::from_rgb(100, 150, 255),
            link_underline_color: Color32::from_rgb(100, 150, 255),
            underline_color: Color32::from_rgb(200, 200, 200),
            header_color: Color32::LIGHT_BLUE,
            list_marker_color: Color32::from_rgb(150, 150, 150),
            task_pending_color: Color32::from_rgb(200, 200, 200),
            task_done_color: Color32::from_rgb(100, 200, 100),
        }
    }
}

use egui::{TextFormat, FontId, FontFamily, Color32, text::LayoutJob};