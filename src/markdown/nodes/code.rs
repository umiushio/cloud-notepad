use super::*;

/// 代码块节点
pub(super) struct CodeBlock {
    language: Option<String>,
    content: Vec<Range<usize>>,
}

impl Node for CodeBlock {
    fn parse(input: &str, pos: usize) -> Option<ParseResult<Self>>
            where Self: Sized {
        // 检查是否以 ``` 开头
        if input[pos..].starts_with("```") {
            // 之后有 \n 就判定为代码块
            if let Some(lang_end) = input[pos+3..].find('\n').map(|i| pos + 3 + i) {
                // 先解析语言
                let language = if lang_end > 0 {
                    Some(input[pos+3..lang_end].trim().to_string())
                } else {
                    None
                };

                // 计算代码块范围，需要匹配正则表达式
                let re = regex::Regex::new(r"\n```\s*\n").unwrap();
                let (code_end, block_end) = re
                    .find(&input[pos..])
                    .map(|mat| (pos + mat.start() + 1, pos + mat.end()))
                    .unwrap_or((input.len(), input.len()));
                
                // 将内容按换行分块
                let code_start = lang_end + 1;
                let code_slice = &input[code_start..code_end];
                let mut line_start = code_start;
                let mut content = Vec::new();
                for (i, c) in code_slice.char_indices() {
                    if c == '\n' {
                        // 包含换行符
                        let line_end = code_start + i + 1;
                        content.push(line_start..line_end);
                        line_start = line_end;
                    }
                }
                // 处理最后一行
                if line_start < code_end {
                    content.push(line_start..code_end);
                }

                return Some(ParseResult {
                    node: Self { language, content },
                    end: block_end,
                })
            }
        }

        None
    }
    
    fn render(&self, job: &mut LayoutJob, ctx: &RenderContext, front_width: f32) {
        for (i, range) in self.content.iter().enumerate() {
            let line = &ctx.text[range.clone()];
            let leading_space = if i == 0 { 0.0 } else { front_width };
            job.append(line, leading_space, TextFormat {
                font_id: FontId::monospace(13.0),
                color: ctx.theme.code_color,
                background: ctx.theme.code_bg_color,
                ..Default::default()
            });
        }
    }
}
