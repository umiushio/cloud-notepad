use super::{Node, Theme, RenderContext, nodes::parse_node};
use egui::text::LayoutJob;

/// Markdown渲染器
pub struct MarkdownRenderer<'a> {
    text: &'a str,
    nodes: Vec<Box<dyn Node>>,
    theme: Theme,
}

impl<'a> MarkdownRenderer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, nodes: Vec::new(), theme: Theme::default() }
    }

    fn parse(&mut self) {
        let mut pos = 0;
        while pos < self.text.len() {
            // 尝试按优先级解析各种节点类型
            if let Some((node, end)) = parse_node(self.text, pos) {
                self.nodes.push(node);
                pos = end;
            } else {
                unreachable!()
                // // 安全前进（避免无限循环）
                // self.pos += 1;
            }
        }
    }

    pub fn render(&mut self, cursor_pos: usize) -> LayoutJob {
        // 先执行一遍解析
        self.parse();

        // 开始渲染
        let mut job = LayoutJob::default();
        let ctx = RenderContext {
            text: self.text,
            cursor_pos,
            theme: self.theme.clone(),
        };

        for node in self.nodes.iter() {
            node.render(&mut job, &ctx, 0.0);
        }

        job
    }
}