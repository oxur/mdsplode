use markdown::mdast::Node;

use serde::{Deserialize, Serialize};

use crate::md;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompoundChildren {
    pub nodes: Vec<CompoundNode>,
    pub html: String,
    pub json: String,
}

impl CompoundChildren {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompoundNode {
    pub name: String,
    pub depth: i8,
    pub source: String,
    pub children: CompoundChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
    #[serde(skip_serializing)]
    node: Node,
}

impl CompoundNode {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn parse_node(node: &Node) -> CompoundNode {
    let node_chs = to_children(node);
    let chs = CompoundChildren {
        nodes: node_chs.clone(),
        json: render_json_nodes(node_chs.clone()),
        html: render_html_nodes(node_chs),
    };
    let src = node.to_string();
    let mut c_node = CompoundNode {
        node: node.clone(),
        name: md::node_type(node.clone()),
        depth: md::node_depth(node.clone()),
        source: src,
        html: None,
        json: None,
        children: chs.clone(),
    };
    if chs.is_empty() {
        c_node.html = render_html(node.clone());
        c_node.json = render_json(node.clone());
    };
    c_node
}

pub fn parse_file(file: &str) -> CompoundNode {
    parse_node(&md::converter::file_to_mdast(file))
}

pub fn to_children(node: &Node) -> Vec<CompoundNode> {
    match node.children() {
        None => vec![],
        Some(ch) => ch.iter().map(parse_node).collect::<Vec<CompoundNode>>(),
    }
}

pub fn render_html_nodes(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| match render_html(x.node.clone()) {
            Some(y) => y,
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn render_json_nodes(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| match render_json(x.node.clone()) {
            Some(y) => y,
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn render_html(n: Node) -> Option<String> {
    match n {
        Node::Toml(_) => None,
        _ => Some(md::converter::markdown_to_html(n.clone())),
    }
}

pub fn render_json(n: Node) -> Option<String> {
    match n {
        Node::Toml(_) => Some(md::converter::toml_to_json(n.clone())),
        _ => None,
    }
}
