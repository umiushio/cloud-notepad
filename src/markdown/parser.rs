use super::*;

pub(super) struct MarkdownParser<'a> {
    input: &'a str,
    pos: usize,
    nodes: Vec<Box<dyn Node>>,
}

impl<'a> MarkdownParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { nodes: Vec::new(), input, pos: 0 }
    }

    fn parse(&mut self) { 
        while self.pos < self.input.len() {
            // 尝试按优先级解析各种节点类型
            if let Some((node, end)) = nodes::parse_node(self.input, self.pos) {
                self.nodes.push(node);
                self.pos = end;
            } else {
                unreachable!()
                // // 安全前进（避免无限循环）
                // self.pos += 1;
            }
        }
    }

    pub fn as_nodes(&mut self) -> &Vec<Box<dyn Node>> {
        self.parse();
        &self.nodes
    }
}