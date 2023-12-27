pub mod convert;
pub mod opts;

use markdown::mdast::Node;

pub fn node_type(n: Node) -> String {
    match n {
        Node::Root(_) => "root".to_string(),
        Node::BlockQuote(_) => "blockquote".to_string(),
        Node::FootnoteDefinition(_) => "footnote".to_string(),
        Node::MdxJsxFlowElement(_) => "mdxjsx-flow-element".to_string(),
        Node::List(_) => "list".to_string(),
        Node::MdxjsEsm(_) => "mdxjsesm".to_string(),
        Node::Toml(_) => "toml".to_string(),
        Node::Yaml(_) => "yaml".to_string(),
        Node::Break(_) => "break".to_string(),
        Node::InlineCode(_) => "inline-code".to_string(),
        Node::InlineMath(_) => "inline-math".to_string(),
        Node::Delete(_) => "delete".to_string(),
        Node::Emphasis(_) => "emphasis".to_string(),
        Node::MdxTextExpression(_) => "mdx-text".to_string(),
        Node::FootnoteReference(_) => "footnote-ref".to_string(),
        Node::Html(_) => "html".to_string(),
        Node::Image(_) => "image".to_string(),
        Node::ImageReference(_) => "image-ref".to_string(),
        Node::MdxJsxTextElement(_) => "mdxjsx-text".to_string(),
        Node::Link(_) => "link".to_string(),
        Node::LinkReference(_) => "link-ref".to_string(),
        Node::Strong(_) => "strong".to_string(),
        Node::Text(_) => "text".to_string(),
        Node::Code(_) => "code".to_string(),
        Node::Math(_) => "math".to_string(),
        Node::MdxFlowExpression(_) => "mdx-flow".to_string(),
        Node::Heading(_) => "heading".to_string(),
        Node::Table(_) => "table".to_string(),
        Node::ThematicBreak(_) => "thematic-break".to_string(),
        Node::TableRow(_) => "table-row".to_string(),
        Node::TableCell(_) => "table-cell".to_string(),
        Node::ListItem(_) => "list-item".to_string(),
        Node::Definition(_) => "definition".to_string(),
        Node::Paragraph(_) => "paragraph".to_string(),
    }
}

pub fn node_depth(n: Node) -> i8 {
    match n {
        Node::Heading(h) => h.depth as i8,
        _ => -1,
    }
}
