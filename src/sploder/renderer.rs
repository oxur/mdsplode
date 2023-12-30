use markdown::mdast::Node;

use crate::md;

use super::types::CompoundNode;

pub fn html_nodes(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| match html(x.node.clone()) {
            Some(y) => y,
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn json_nodes(nodes: Vec<CompoundNode>) -> String {
    nodes
        .iter()
        .map(|x| match json(x.node.clone()) {
            Some(y) => y,
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn html(n: Node) -> Option<String> {
    match n {
        Node::Toml(_) => None,
        _ => Some(md::converter::markdown_to_html(n.clone())),
    }
}

pub fn json(n: Node) -> Option<String> {
    match n {
        Node::Toml(_) => Some(md::converter::toml_to_json(n.clone())),
        _ => None,
    }
}
