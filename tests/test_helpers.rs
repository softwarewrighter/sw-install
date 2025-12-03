// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Shared test utilities for sw-install integration tests.

use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Creates a simple Cargo project with optional compiled binary.
pub fn create_temp_project(dir: &Path, include_binary: bool) -> std::io::Result<()> {
    let cargo_toml = dir.join("Cargo.toml");
    fs::write(
        cargo_toml,
        r#"[package]
name = "test-app"
version = "0.1.0"
edition = "2021"
"#,
    )?;

    if include_binary {
        let target_dir = dir.join("target").join("release");
        fs::create_dir_all(&target_dir)?;
        fs::write(target_dir.join("test-app"), "fake binary")?;
    }

    Ok(())
}

/// Creates a workspace Cargo.toml with the specified members.
pub fn create_workspace_cargo_toml(dir: &Path, members: &str) -> std::io::Result<()> {
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[workspace]
resolver = "2"
members = {members}
"#
        ),
    )
}

/// Creates a package Cargo.toml with optional binary section.
pub fn create_package_cargo_toml(dir: &Path, name: &str, bin: Option<&str>) -> std::io::Result<()> {
    let bin_section = bin
        .map(|n| {
            format!(
                r#"
[[bin]]
name = "{n}"
path = "src/main.rs"
"#
            )
        })
        .unwrap_or_default();
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
{bin_section}"#
        ),
    )
}

/// Creates a library crate (no main.rs).
pub fn create_lib_crate(dir: &Path, name: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("src"))?;
    create_package_cargo_toml(dir, name, None)?;
    fs::write(dir.join("src").join("lib.rs"), "pub fn foo() {}")
}

/// Creates a binary crate with main.rs.
pub fn create_bin_crate(dir: &Path, name: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("src"))?;
    create_package_cargo_toml(dir, name, Some(name))?;
    fs::write(dir.join("src").join("main.rs"), "fn main() {}")
}

/// Creates a test binary directory with the specified binaries.
pub fn create_test_bin_dir(temp: &TempDir) -> std::path::PathBuf {
    let bin_dir = temp.path().join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    bin_dir
}

/// Creates fake binaries in the specified directory.
pub fn create_fake_binaries(dir: &Path, names: &[&str]) -> std::io::Result<()> {
    for name in names {
        fs::write(dir.join(name), "fake binary")?;
    }
    Ok(())
}
