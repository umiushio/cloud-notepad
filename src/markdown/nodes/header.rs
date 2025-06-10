use super::*;

/// 标题节点
pub(super) struct Header {
    level: u8,
    content: Range<usize>,
}

impl Node for Header {
    fn parse(input: &str, pos: usize) -> Option<ParseResult<Self>>
            where Self: Sized {
        // 检查以 # 开头
        if input[pos..].starts_with('#') {
            // 先确定结束位置，即第一个换行结束位置（或者文本末尾）
            let line_end = input[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(input.len());
            // 得到标题等级（最大为6）
            let level = input[pos..line_end]
                .chars()
                .take_while(|&c| c == '#')
                .count()
                .min(6) as u8;

            return Some(ParseResult { 
                node: Self { level, content: pos + level as usize..line_end }, 
                end: line_end 
            });
        }

        None
    }

    fn render(&self, job: &mut LayoutJob, ctx: &RenderContext, front_width: f32) {
        let _ = front_width;
        let content = &ctx.text[self.content.clone()];
        let font_size = match self.level {
            1 => 24.0,
            2 => 22.0,
            3 => 20.0,
            4 => 18.0,
            _ => 16.0,
        };
        job.append(&content, 0.0, TextFormat { 
            font_id: FontId::proportional(font_size), 
            color: ctx.theme.header_color, 
            underline: egui::Stroke::new(1.0, Color32::BLUE),
            ..Default::default()
        });
    }
}