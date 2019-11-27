use rnix::{SyntaxKind, SyntaxNode};
use std::collections::HashMap;

// hashmap of attributes by ident key and expr text value
pub type AttrHashMap = HashMap<String, String>;

pub fn attr_set_binds_to_hashmap(node: &SyntaxNode) -> AttrHashMap {
    let mut hm = HashMap::new();

    let binds = node.children();
    for bind in binds {
        if let Some(pair) = bind_to_option_pair(&bind) {
            hm.insert(pair.key, pair.value);
        }
    }

    hm
}

#[derive(Debug)]
pub struct Pair {
    key: String,
    value: String,
}

pub fn bind_to_option_pair(node: &SyntaxNode) -> Option<Pair> {
    let mut children = node.children();
    let key = children.next().and_then(|x| get_key_ident_string(&x))?;
    let value = children.next().and_then(|x| get_string_string(&x))?;

    Some(Pair { key, value })
}

pub fn get_node_attr_set(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_ATTR_SET => Some(node),
        _ => None,
    }
}

pub fn get_node_key(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_KEY => Some(node),
        _ => None,
    }
}

pub fn get_node_string(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_STRING => Some(node),
        _ => None,
    }
}

pub fn node_to_string(node: &SyntaxNode) -> String {
    node.text().to_string()
}

pub fn get_key_ident_string(node: &SyntaxNode) -> Option<String> {
    get_node_key(node).map(node_to_string)
}

pub fn get_string_string(node: &SyntaxNode) -> Option<String> {
    get_node_string(node).map(node_to_string)
}
