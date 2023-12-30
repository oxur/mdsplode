use markdown::mdast::Node;
use serde::{Deserialize, Serialize};

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
    pub node: Node,
}

impl CompoundNode {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
