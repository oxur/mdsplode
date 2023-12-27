use markdown::mdast::Node;

use serde::{Deserialize, Serialize};

use crate::md;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompoundChildren {
    pub nodes: Vec<CompoundNode>,
    pub rendered: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompoundNode {
    pub name: String,
    pub depth: i8,
    pub markdown: String,
    pub children: CompoundChildren,
    pub rendered: String,
    #[serde(skip_serializing)]
    node: Node,
}

impl CompoundNode {
    pub fn traverse(&mut self) {
        for ch in self.children.nodes.iter_mut() {
            ch.traverse();
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

pub fn parse_node(node: &Node) -> CompoundNode {
    let children = to_children(node);
    let md = node.to_string();
    let html = render_node(node.clone());
    CompoundNode {
        node: node.clone(),
        name: md::node_type(node.clone()),
        depth: md::node_depth(node.clone()),
        markdown: md,
        rendered: html,
        children: CompoundChildren {
            nodes: children.clone(),
            rendered: render_nodes(children),
        },
    }
}

pub fn parse_file(file: &str) -> CompoundNode {
    parse_node(&md::convert::file_to_mdast(file))
}

pub fn to_children(node: &Node) -> Vec<CompoundNode> {
    match node.children() {
        None => vec![],
        Some(ch) => ch.iter().map(parse_node).collect::<Vec<CompoundNode>>(),
    }
}

pub fn render_nodes(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| render_node(x.node.clone()))
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn render_node(n: Node) -> String {
    match n {
        Node::Toml(_) => md::convert::node_to_toml(n),
        _ => md::convert::node_to_html(n),
    }
}
