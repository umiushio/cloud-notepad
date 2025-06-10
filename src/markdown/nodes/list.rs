use super::*;

/// 列表树节点
pub(super) struct ListNode {
    header: ListTree,
}

/// 列表根节点
enum ListTree {
    Branch {
        children: Vec<Box<ListTree>>,
        indent: usize,
        list_type: ListType,
    },
    Leaf {
        node: Box<dyn Node>,
        indent: usize,
    },
}

/// 列表类型
enum ListType {
    Ordered(usize),
    Unordered,
    Task(bool),
}

/// 列表解析上下文
struct ParseListContext {

}

impl Node for ListNode {
    fn parse(text: &str, pos: usize) -> Option<ParseResult<Self>> {
        None
    }

    fn render(&self, job: &mut LayoutJob, ctx: &RenderContext, front_width: f32) {
        
    }

}

impl ListTree {
    fn dfs_render(&self, job: &mut LayoutJob, text: &str, ctx: &RenderContext) {
        
    }
}