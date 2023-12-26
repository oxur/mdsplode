use std::fs;

use markdown::mdast::Node;
use markdown::{mdast, CompileOptions, Constructs, Options, ParseOptions};
use serde::{Deserialize, Serialize};

fn get_constructs() -> Constructs {
    Constructs {
        frontmatter: true,
        ..Constructs::gfm()
    }
}

fn get_options() -> Options {
    Options {
        parse: ParseOptions {
            constructs: get_constructs(),
            ..ParseOptions::gfm()
        },
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::gfm()
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct CompoundChildren {
    nodes: Vec<CompoundNode>,
    rendered: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct CompoundNode {
    // node: mdast::Node,
    name: String,
    depth: i8,
    markdown: String,
    children: CompoundChildren,
    rendered: String,
}

impl CompoundNode {
    fn traverse(&mut self) {
        for ch in self.children.nodes.iter_mut() {
            ch.traverse();
        }
    }
}

fn to_compound(node: &mdast::Node) -> CompoundNode {
    let children = to_children(node);
    let md = node.to_string();
    let html = markdown::to_html_with_options(md.as_str(), &get_options()).unwrap();
    CompoundNode {
        // node: node.clone(),
        name: node_type(node.clone()),
        depth: node_depth(node.clone()),
        markdown: md,
        rendered: html,
        children: CompoundChildren {
            nodes: children.clone(),
            rendered: markdown::to_html_with_options(to_string(children).as_str(), &get_options())
                .unwrap(),
        },
    }
}

fn to_children(node: &mdast::Node) -> Vec<CompoundNode> {
    match node.children() {
        None => vec![],
        Some(ch) => ch.iter().map(to_compound).collect::<Vec<CompoundNode>>(),
    }
}

fn to_string(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| x.rendered.clone())
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn node_type(n: Node) -> String {
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

fn node_depth(n: Node) -> i8 {
    match n {
        Node::Heading(h) => h.depth as i8,
        _ => -1,
    }
}

fn main() {
    let file = "./workbench/learn.md";
    let md = fs::read_to_string(file).unwrap();
    let mut tree = to_compound(&markdown::to_mdast(&md, &ParseOptions::gfm()).unwrap());
    tree.traverse();
    let json = serde_json::to_string_pretty(&tree);
    println!("{:}", json.unwrap());
}
