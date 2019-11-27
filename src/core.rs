use rnix::SyntaxNode;
use serde::{Deserialize, Serialize};
use std::process;

use crate::node::AttrHashMap;

pub enum ReplacementMethod {
    FetchTarball { url: String },
    FetchUrl { url: String },
    FetchFromGitHub { owner: String, repo: String },
    Fetchgit { url: String },
}

pub type ReplacementString = String;

pub fn handle_fetch(node: &SyntaxNode, attrs: &AttrHashMap) -> Option<ReplacementMethod> {
    handle_fetch_tarball(node, attrs)
        .or_else(|| handle_fetch_url(node, attrs))
        .or_else(|| handle_fetch_github(node, attrs))
        .or_else(|| handle_fetchgit(node, attrs))
}

pub fn prepare_replacement(method: &ReplacementMethod) -> Option<ReplacementString> {
    match method {
        ReplacementMethod::FetchTarball { url } => prepare_tarball_replacement(url),
        ReplacementMethod::FetchUrl { url } => prepare_url_replacement(url),
        ReplacementMethod::FetchFromGitHub { owner, repo } => {
            prepare_github_replacement(owner, repo)
        }
        ReplacementMethod::Fetchgit { url } => prepare_git_replacement(url),
    }
}

// if there's a fetchTarball call, get a new sha for it.
fn handle_fetch_tarball(node: &SyntaxNode, attrs: &AttrHashMap) -> Option<ReplacementMethod> {
    let prev_node_string: String = node.prev_sibling()?.text().to_string();
    let prev_contains_fetch_tarball = prev_node_string.contains("fetchTarball");

    if !prev_contains_fetch_tarball {
        return None;
    }

    let url = attrs.get("url")?;
    let _sha256 = attrs.get("sha256")?;

    Some(ReplacementMethod::FetchTarball {
        url: url.to_owned(),
    })
}

fn prepare_tarball_replacement(url: &String) -> Option<ReplacementString> {
    let prefetch_attempt = process::Command::new("nix-prefetch-url")
        .arg(remove_quotes(url))
        .arg("--unpack")
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

// if there's a fetchurl call, get a new sha for it.
fn handle_fetch_url(node: &SyntaxNode, attrs: &AttrHashMap) -> Option<ReplacementMethod> {
    let prev_node_string: String = node.prev_sibling()?.text().to_string();
    let prev_contains_fetch_url = prev_node_string.contains("fetchurl");

    if !prev_contains_fetch_url {
        return None;
    }

    let url = attrs.get("url")?;
    let _sha256 = attrs.get("sha256")?;

    Some(ReplacementMethod::FetchUrl {
        url: url.to_owned(),
    })
}

fn prepare_url_replacement(url: &String) -> Option<ReplacementString> {
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
fn handle_fetch_github(node: &SyntaxNode, attrs: &AttrHashMap) -> Option<ReplacementMethod> {
    let prev_node_string: String = node.prev_sibling()?.text().to_string();
    let prev_contains_fetch_github = prev_node_string.contains("fetchFromGitHub");

    if !prev_contains_fetch_github {
        return None;
    }

    let owner = attrs.get("owner")?;
    let repo = attrs.get("repo")?;
    let _rev = attrs.get("rev")?;
    let _sha256 = attrs.get("sha256")?;

    Some(ReplacementMethod::FetchFromGitHub {
        owner: owner.to_owned(),
        repo: repo.to_owned(),
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct PrefetchGit {
    rev: String,
    sha256: String,
}

fn prepare_github_replacement(owner: &String, repo: &String) -> Option<ReplacementString> {
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

// if there's a fetchgit call, get the newest reversion and sha for it.
fn handle_fetchgit(node: &SyntaxNode, attrs: &AttrHashMap) -> Option<ReplacementMethod> {
    let prev_node_string: String = node.prev_sibling()?.text().to_string();
    let prev_contains_fetchgit = prev_node_string.contains("fetchgit");

    if !prev_contains_fetchgit {
        return None;
    }

    let url = attrs.get("url")?;
    let _rev = attrs.get("rev")?;
    let _sha256 = attrs.get("sha256")?;

    Some(ReplacementMethod::Fetchgit {
        url: url.to_owned(),
    })
}

fn prepare_git_replacement(url: &String) -> Option<ReplacementString> {
    let prefetch_attempt = process::Command::new("nix-prefetch-git")
        .arg(format!("{}", remove_quotes(url)))
        .output()
        .expect("Error: Failed to launch nix-prefetch-git.");

    if prefetch_attempt.status.success() {
        let json = String::from_utf8(prefetch_attempt.stdout).unwrap();
        let prefetch: PrefetchGit = serde_json::from_str(&json).unwrap();

        println!("Fetched rev and sha256 for {}", url);
        Some(format!(
            r#"{{
                url = {};
                rev = "{}";
                sha256 = "{}";
            }}"#,
            url, prefetch.rev, prefetch.sha256,
        ))
    } else {
        println!("Failed to prefetch git repo: {}", url);
        None
    }
}

fn remove_quotes(string: &String) -> String {
    string.replace("\"", "")
}
