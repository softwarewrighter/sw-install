// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Cargo workspace utilities for sw-install.

use std::fs;
use std::path::{Path, PathBuf};

pub fn find_workspace_binaries(root: &Path, members: &[toml::Value]) -> Vec<String> {
    members
        .iter()
        .filter_map(|m| m.as_str())
        .flat_map(|member| expand_member_paths(root, member))
        .flat_map(|path| extract_binaries_from_member(root, &path))
        .collect()
}

fn expand_member_paths(root: &Path, member: &str) -> Vec<PathBuf> {
    if let Some(base) = member.strip_suffix("/*") {
        fs::read_dir(root.join(base))
            .map(|e| {
                e.filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| PathBuf::from(base).join(e.file_name()))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        vec![PathBuf::from(member)]
    }
}

fn extract_binaries_from_member(root: &Path, path: &Path) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(root.join(path).join("Cargo.toml")) else {
        return vec![];
    };
    let Ok(value) = toml::from_str::<toml::Value>(&contents) else {
        return vec![];
    };
    if let Some(bins) = value.get("bin").and_then(|b| b.as_array()) {
        return bins
            .iter()
            .filter_map(|b| b.get("name").and_then(|n| n.as_str()))
            .map(String::from)
            .collect();
    }
    if let Some(pkg) = value.get("package")
        && let Some(name) = pkg.get("name").and_then(|n| n.as_str())
        && root.join(path).join("src/main.rs").exists()
    {
        return vec![name.to_string()];
    }
    vec![]
}
