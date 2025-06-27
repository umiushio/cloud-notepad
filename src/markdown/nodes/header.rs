use super::*;
use super::text::TextNode;

/// 标题节点
pub(super) struct Header {
    level: u8,
    content: TextNode,
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
            // 得到标题文本开始位置，需要忽略空白字符
            let header_text_pos = pos + level as usize;

            return Some(ParseResult { 
                node: Self { level, content: TextNode::parse(input, header_text_pos).unwrap().node }, 
                end: line_end 
            });
        }

        None
    }

    fn render(&self, job: &mut LayoutJob, config: &RenderConfig, ctx: Option<RenderContext>) {
        let _ = ctx;
        let font_size = match self.level {
            1 => 24.0,
            2 => 22.0,
            3 => 20.0,
            4 => 18.0,
            _ => 16.0,
        };
        self.content.render(job, config, Some(RenderContext { front_width: 0.0, font_size }));
    }
}