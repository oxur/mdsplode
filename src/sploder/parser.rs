use markdown::mdast::Node;

use crate::md;

use super::renderer;
use super::types::{CompoundChildren, CompoundNode};

pub fn parse_node(node: &Node) -> CompoundNode {
    let node_chs = to_children(node);
    let chs = CompoundChildren {
        nodes: node_chs.clone(),
        json: renderer::json_nodes(node_chs.clone()),
        html: renderer::html_nodes(node_chs),
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
        c_node.html = renderer::html(node.clone());
        c_node.json = renderer::json(node.clone());
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
