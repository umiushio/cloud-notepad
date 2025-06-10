use super::*;

/// 文本段
#[derive(Debug, Clone)]
enum TextSegment {
    Plain(Range<usize>),
    Bold(Range<usize>),
    Italic(Range<usize>),
    Underline(Range<usize>),
    Code(Range<usize>),
    Link { text: Range<usize>, url: Range<usize> },
}

impl Node for TextSegment {
    fn parse(input: &str, pos: usize) -> Option<ParseResult<Self>>
            where Self: Sized {
        let n = input.len();
        if n <= pos { return None; }

        let remaining = &input[pos..];
        let mut mark = 0;

        // 检测链接 [text](url)
        if remaining.starts_with('[') {
            mark = 1;
            if let Some(text_end) = remaining.find("](") {
                if text_end > 0 {
                    if let Some(url_end) = remaining[text_end+2..].find(')') {
                        let text_range = pos+1..pos+text_end;
                        let url_range = pos+text_end+2..pos+text_end+2+url_end;
                        return Some(ParseResult { 
                            node: Self::Link { text: text_range, url: url_range }, 
                            end: pos+text_end+3+url_end,
                        });
                    }
                }
            } 
        }

        // 检测行内代码 `code`
        if remaining.starts_with('`') {
            mark = 1;
            if let Some(end) = remaining[1..].find('`') {
                if end > 0 {
                    return Some(ParseResult { 
                        node: Self::Code(pos+1..pos+1+end), 
                        end: pos+end+2, 
                    });
                }
            }
        }

        // 检测粗体 **bold** 或 __bold__
        if remaining.starts_with("**") || remaining.starts_with("__") {
            let re = regex::Regex::new(format!(r"\S+{}", regex::escape(&remaining[..2])).as_str()).unwrap();
            if let Some(mat) = re.find(&remaining[2..]) {
                return Some(ParseResult {
                    node: Self::Bold(pos+2..pos+mat.end()),
                    end: pos+2+mat.end(),
                });
            }
        }

        // 检测斜体 *italic* 或 _italic_
        if remaining.starts_with('*') || remaining.starts_with('_') {
            mark = 1;
            let re = regex::Regex::new(format!(r"\S+{}", regex::escape(&remaining[..1])).as_str()).unwrap();
            if let Some(mat) = re.find(&remaining[1..]) {
                return Some(ParseResult {
                    node: Self::Bold(pos+1..pos+mat.end()),
                    end: pos+1+mat.end(),
                });
            }
        }

        // 检测下划线 ~underline~
        if remaining.starts_with('~') {
            mark = 1;
            if let Some(end) = remaining[1..].find('~') {
                if end > 0 {
                    return Some(ParseResult {
                        node: Self::Underline(pos+1..pos+1+end),
                        end: pos+1+end+1
                    });
                }
            }
        }

        // mark=1时必定为特定字符，否则mark=0，避免中文字符影响
        let end = remaining[mark..]
            .find(|c| matches!(c, '[' | '`' | '*' | '_' | '~'))
            .map(|i| pos + i + mark)
            .unwrap_or(n);
        
        Some(ParseResult {
            node: Self::Plain(pos..end),
            end,
        })
    }

    fn render(&self, job: &mut LayoutJob, ctx: &RenderContext, front_width: f32) {
        let _ = front_width;
        match self {
            Self::Plain(range) => {
                let content = &ctx.text[range.clone()];
                job.append(content, 0.0, TextFormat::simple(
                    FontId::proportional(14.0), 
                    if range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.text_color }
                ));
            }
            Self::Bold(range) => {
                let content = &ctx.text[range.clone()];
                job.append(content, 0.0, TextFormat {
                    font_id: FontId::new(14.0, FontFamily::Name("Bold".into())),
                    color: if range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.bold_color },
                    ..Default::default()
                });
            }
            Self::Italic(range) => {
                let content = &ctx.text[range.clone()];
                job.append(content, 0.0, TextFormat {
                    font_id: FontId::proportional(14.0),
                    color: if range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.text_color },
                    italics: true,
                    ..Default::default()
                });
            }
            Self::Underline(range) => {
                let content = &ctx.text[range.clone()];
                job.append(content, 0.0, TextFormat {
                    font_id: FontId::proportional(14.0),
                    color: if range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.text_color },
                    underline: egui::Stroke::new(1.0, ctx.theme.underline_color),
                    ..Default::default()
                });
            }
            Self::Code(range) => {
                let content = &ctx.text[range.clone()];
                job.append(content, 0.0, TextFormat {
                    font_id: FontId::monospace(13.0),
                    color: if range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.code_color },
                    background: ctx.theme.code_bg_color,
                    ..Default::default()
                });
            }

            Self::Link { text: text_range, url: _ } => {
                let content = &ctx.text[text_range.clone()];
                job.append(content, 0.0, TextFormat {
                    font_id: FontId::proportional(14.0),
                    color: if text_range.contains(&ctx.cursor_pos) { Color32::WHITE } else { ctx.theme.link_color },
                    underline: egui::Stroke::new(1.0, ctx.theme.link_underline_color),
                    ..Default::default()
                });
            }
        }
    }

    // fn range() -> Range() {

    // }
}

/// 纯文本节点
pub(super) struct TextNode {
    segments: Vec<TextSegment>,
}

impl Node for TextNode {
    fn parse(input: &str, pos: usize) -> Option<ParseResult<Self>> {
        if pos >= input.len() { return None; }
        let mut segments = Vec::new();
        // 收集一行的所有内容（包含换行）
        let line_end = input[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(input.len());

        let line = &input[..line_end];
        let mut line_pos = pos;
        // 解析当行中的内联元素
        while line_pos < line_end {
            let segment = TextSegment::parse(line, line_pos).unwrap();
            segments.push(segment.node);
            line_pos = segment.end;
        }
        Some(ParseResult {
            node: Self { segments },
            end: line_end,
        })
    }

    fn render(&self, job: &mut egui::text::LayoutJob, ctx: &RenderContext, front_width: f32) {
        let _ = front_width;
        for segment in self.segments.iter() {
            segment.render(job, ctx, 0.0);
        }
    }
}