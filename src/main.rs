use rnix::{SyntaxKind, SyntaxNode, TextRange};
use std::{collections::HashMap, fs, io, path::Path};

fn main() {
    let path = Path::new("./test/fetch-url.nix");
    let result = main1(path);
    println!("main1 result: {:?}", result);
}

fn main1(file: &Path) -> io::Result<()> {
    let input = fs::read_to_string(file)?;

    let ast = rnix::parse(&input);
    let root_node = ast.node();
    let result = format(&root_node).to_string();

    let output = nixpkgs_fmt::reformat_string(&result);
    if input != output {
        fs::write(file, &output)?;
    }
    Ok(())
}

use nixpkgs_fmt::{
    engine::fmt_model::FmtModel, tree_utils::walk_non_whitespace, AtomEdit, FmtDiff,
};

/// The main entry point for formatting
fn format(root: &SyntaxNode) -> FmtDiff {
    let mut model = FmtModel::new(root.clone());
    for element in walk_non_whitespace(root) {
        element
            .as_node()
            .and_then(|x| get_node_attr_set(&x))
            .map(|node| {
                let set = attr_set_binds_to_hashmap(&node);
                println!("attr set: {:?}", set);

                // example edit
                let range = node.clone().text_range();
                let delete = TextRange::offset_len(range.start(), range.len());
                let insert = "replacement".into();
                model.raw_edit(AtomEdit { delete, insert });
            });
    }

    // model.into_diff()
    FmtModel::new(root.clone()).into_diff()
}

fn attr_set_binds_to_hashmap(node: &SyntaxNode) -> HashMap<String, String> {
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
struct Pair {
    key: String,
    value: String,
}

fn bind_to_option_pair(node: &SyntaxNode) -> Option<Pair> {
    let mut children = node.children();
    let key = children.next().and_then(|x| get_key_ident_string(&x))?;
    let value = children.next().and_then(|x| get_string_string(&x))?;

    Some(Pair { key, value })
}

fn get_node_attr_set(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_ATTR_SET => Some(node),
        _ => None,
    }
}

fn get_node_key(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_KEY => Some(node),
        _ => None,
    }
}

fn get_node_string(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind() {
        SyntaxKind::NODE_STRING => Some(node),
        _ => None,
    }
}

fn node_to_string(node: &SyntaxNode) -> String {
    node.text().to_string()
}

fn get_key_ident_string(node: &SyntaxNode) -> Option<String> {
    get_node_key(node).map(node_to_string)
}

fn get_string_string(node: &SyntaxNode) -> Option<String> {
    get_node_string(node).map(node_to_string)
}
