pub mod core;
pub mod node;

use nixpkgs_fmt::{
    engine::fmt_model::FmtModel, tree_utils::walk_non_whitespace, AtomEdit, FmtDiff,
};
use rayon::prelude::*;
use rnix::{SyntaxNode, TextRange};

pub struct Replacement {
    pub delete: TextRange,
    pub method: core::ReplacementMethod,
}

/// The main entry point for formatting
pub fn format(root: &SyntaxNode) -> FmtDiff {
    let mut model = FmtModel::new(root.clone());

    let replacements: Vec<Replacement> = walk_non_whitespace(root)
        .flat_map(|element| {
            element
                .as_node()
                .and_then(|x| node::get_node_attr_set(&x))
                .and_then(|node: &SyntaxNode| {
                    let set = node::attr_set_binds_to_hashmap(&node);
                    core::handle_fetch(&node, &set).map(|method| {
                        let range = node.text_range();
                        let delete = TextRange::offset_len(range.start(), range.len());
                        Replacement { delete, method }
                    })
                })
        })
        .collect();

    let edits: Vec<AtomEdit> = replacements
        .par_iter()
        .flat_map(|r| {
            core::prepare_replacement(&r.method).and_then(|string| {
                Some(AtomEdit {
                    delete: r.delete,
                    insert: string.into(),
                })
            })
        })
        .collect();

    for edit in edits {
        model.raw_edit(edit);
    }

    model.into_diff()
}
