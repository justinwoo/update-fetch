use rayon::prelude::*;
use rnix::{SyntaxKind, SyntaxNode, TextRange};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, io, path::Path, process};

#[derive(Serialize, Deserialize, Debug)]
struct PrefetchGit {
    rev: String,
    sha256: String,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.is_empty() {
        println!("{}", EXPECT_FILE_PATH_ARG_MSG.trim());
        process::exit(1);
    }

    args.par_iter()
        .map(|x| {
            let path = Path::new(x);
            let result = main1(path);
            match result {
                Ok(_) => println!("Finished {}", x),
                Err(e) => {
                    println!("Error on {}: {}", x, e);
                }
            }
        })
        .collect::<()>();
}

const EXPECT_FILE_PATH_ARG_MSG: &str = r#"
Need arguments for what in_files to process.
Usage Examples:
    # update a single in_file
    update-fetch-derivation my-in_file.nix
    # Using fd (sequential)
    fd -e nix -x update-fetch-derivation {}
    # Multiple files
    update-fetch-derivation *.nix
"#;

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

    let mut add_replacement = |node: &SyntaxNode, replacement: &String| {
        let range = node.clone().text_range();
        let delete = TextRange::offset_len(range.start(), range.len());
        let insert = replacement.into();
        model.raw_edit(AtomEdit { delete, insert });
    };

    for element in walk_non_whitespace(root) {
        element
            .as_node()
            .and_then(|x| get_node_attr_set(&x))
            .map(|node| {
                let set = attr_set_binds_to_hashmap(&node);
                if let Some(replacement) = handle_fetch_tarball(&set)
                    .or_else(|| handle_fetch_url(&set))
                    .or_else(|| handle_fetch_github(&set))
                {
                    add_replacement(node, &replacement)
                }
            });
    }

    model.into_diff()
}

// replacement string
type Replacement = String;

// based on heuristic: if it has a name, it's probably used for fetchTarball.
fn handle_fetch_tarball(attrs: &AttrHashMap) -> Option<Replacement> {
    let url = attrs.get("url")?;
    let _sha256 = attrs.get("sha256")?;
    let name = attrs.get("name")?;

    let prefetch_attempt = process::Command::new("nix-prefetch-url")
        .arg(url.replace("\"", ""))
        .arg("--unpack")
        .output()
        .expect("Error: Failed to launch nix-prefetch-url.");

    if prefetch_attempt.status.success() {
        let new_sha256_str = String::from_utf8(prefetch_attempt.stdout).unwrap();
        let new_sha256 = new_sha256_str.trim();
        println!("Fetched sha256 for {}", url);
        Some(format!(
            r#"{{
                name = {};
                url = {};
                sha256 = "{}";
            }}"#,
            name, url, new_sha256
        ))
    } else {
        println!("Failed to prefetch url: {}", url);
        None
    }
}

fn remove_quotes(string: &String) -> String {
    string.replace("\"", "")
}

// if there's a fetchurl call, get a new sha for it.
fn handle_fetch_url(attrs: &AttrHashMap) -> Option<Replacement> {
    let url = attrs.get("url")?;
    let _sha256 = attrs.get("sha256")?;

    let prefetch_attempt = process::Command::new("nix-prefetch-url")
        .arg(remove_quotes(&url))
        .output()
        .expect("Error: Failed to launch nix-prefetch-url.");

    if prefetch_attempt.status.success() {
        let new_sha256_str = String::from_utf8(prefetch_attempt.stdout).unwrap();
        let new_sha256 = new_sha256_str.trim();
        println!("Fetched sha256 for {}", url);
        Some(format!(
            r#"{{
                url = {};
                sha256 = "{}";
            }}"#,
            url, new_sha256
        ))
    } else {
        println!("Failed to prefetch url: {}", url);
        None
    }
}

// if there's a fetchFromGitHub call, get the newest reversion and sha for it.
fn handle_fetch_github(attrs: &AttrHashMap) -> Option<Replacement> {
    let owner = attrs.get("owner")?;
    let repo = attrs.get("repo")?;
    let _rev = attrs.get("rev")?;
    let _sha256 = attrs.get("sha256")?;

    let id = format!("{}/{}", remove_quotes(&owner), remove_quotes(&repo));

    let prefetch_attempt = process::Command::new("nix-prefetch-git")
        .arg(format!("https://github.com/{}", id))
        .output()
        .expect("Error: Failed to launch nix-prefetch-git.");

    if prefetch_attempt.status.success() {
        let json = String::from_utf8(prefetch_attempt.stdout).unwrap();
        let prefetch: PrefetchGit = serde_json::from_str(&json).unwrap();

        println!("Fetched rev and sha256 for {}", id);
        Some(format!(
            r#"{{
                owner = {};
                repo = {};
                rev = "{}";
                sha256 = "{}";
            }}"#,
            owner, repo, prefetch.rev, prefetch.sha256,
        ))
    } else {
        println!("Failed to prefetch GitHub repo: {}", id);
        None
    }
}

// hashmap of attributes by ident key and expr text value
type AttrHashMap = HashMap<String, String>;

fn attr_set_binds_to_hashmap(node: &SyntaxNode) -> AttrHashMap {
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
