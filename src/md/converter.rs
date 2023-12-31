use markdown::mdast::Node;

use super::opts;

pub fn markdown_to_html(node: Node) -> String {
    markdown::to_html_with_options(node.to_string().as_str(), &opts::options()).unwrap()
}

pub fn toml_to_json(node: Node) -> String {
    let table = node.to_string().as_str().parse::<toml::Table>().unwrap();
    let mut wrapper = toml::Table::new();
    wrapper.insert("frontmatter".to_string(), toml::Value::Table(table.clone()));
    serde_json::to_string(&wrapper).unwrap()
}

pub fn string_to_mdast(md: String) -> Node {
    markdown::to_mdast(&md, &opts::parse_options()).unwrap()
}
