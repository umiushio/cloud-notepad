use super::*;
use std::ops::Range;

/// Markdown 节点解析结果
pub(super) struct ParseResult<N> {
    pub node: N,
    pub end: usize,
}

/// Markdown 节点 trait
pub(super) trait Node {
    /// 解析原始文本生成节点
    fn parse(input: &str, pos: usize) -> Option<ParseResult<Self>>
        where Self: Sized;
    
    /// 渲染节点为布局任务
    fn render(&self, job: &mut LayoutJob, ctx: &RenderContext, front_width: f32);

//    /// 返回节点包含的文本范围 
//     fn range(&self) -> Range<usize>;
}

pub(super) fn parse_node(input: &str, pos: usize) -> Option<(Box<dyn Node>, usize)> {
    // 按照优先级依次尝试解析
    if let Some(result) = code::CodeBlock::parse(input, pos) {
        Some((Box::new(result.node), result.end))
    }
    else if let Some(result) = header::Header::parse(input, pos) {
        Some((Box::new(result.node), result.end))
    }
    // else if let Some(result) = list::ListNode::parse(input, pos) {
    //     Some((Box::new(result.node), result.end))
    // }
    else if let Some(result) = text::TextNode::parse(input, pos) {
        Some((Box::new(result.node), result.end))
    }
    else {
        None
    }
}

mod text;
mod list;
mod code;
mod header;
