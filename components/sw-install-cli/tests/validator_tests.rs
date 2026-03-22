// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Integration tests for the Validator module

use std::fs;
use std::path::{Path, PathBuf};
use sw_install::{InstallConfig, InstallError, NormalOutput, Validator};
use tempfile::TempDir;

fn create_test_project(dir: &Path, include_binary: bool) -> std::io::Result<()> {
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

fn create_workspace_cargo_toml(dir: &Path, members: &str) -> std::io::Result<()> {
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

fn create_package_cargo_toml(dir: &Path, name: &str, bin: Option<&str>) -> std::io::Result<()> {
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

fn create_lib_crate(dir: &Path, name: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("src"))?;
    create_package_cargo_toml(dir, name, None)?;
    fs::write(dir.join("src").join("lib.rs"), "pub fn foo() {}")
}

fn create_bin_crate(dir: &Path, name: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir.join("src"))?;
    create_package_cargo_toml(dir, name, Some(name))?;
    fs::write(dir.join("src").join("main.rs"), "fn main() {}")
}

fn new_config(path: PathBuf) -> InstallConfig {
    InstallConfig::new(path, None, vec![], false, false, false, false, None)
}

fn new_config_with_filter(path: PathBuf, bin_filter: Vec<String>) -> InstallConfig {
    InstallConfig::new(path, None, bin_filter, false, false, false, false, None)
}

#[test]
fn test_validate_fails_when_project_path_missing() {
    let config = new_config(PathBuf::from("/nonexistent"));
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::ProjectNotFound(_)
    ));
}

#[test]
fn test_validate_fails_when_cargo_toml_missing() {
    let temp_dir = TempDir::new().unwrap();
    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::CargoTomlNotFound(_)
    ));
}

#[test]
fn test_validate_fails_when_binary_missing() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(temp_dir.path(), false).unwrap();

    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::BinaryNotFound(_)
    ));
}

#[test]
fn test_validate_succeeds_with_valid_project() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(temp_dir.path(), true).unwrap();

    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_ok());
    let binaries = result.unwrap().binaries;
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0].0, "test-app");
}

#[test]
fn test_workspace_with_library_only_members_ignores_libs() {
    let temp_dir = TempDir::new().unwrap();

    create_workspace_cargo_toml(temp_dir.path(), r#"["crates/my-lib", "crates/my-cli"]"#).unwrap();

    let lib_dir = temp_dir.path().join("crates").join("my-lib");
    create_lib_crate(&lib_dir, "my-lib").unwrap();

    let cli_dir = temp_dir.path().join("crates").join("my-cli");
    create_bin_crate(&cli_dir, "my-cli").unwrap();

    // Create the target binary for the workspace
    let target_dir = temp_dir.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("my-cli"), "fake binary").unwrap();

    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_ok());
    let binaries = result.unwrap().binaries;
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0].0, "my-cli");
}

#[test]
fn test_workspace_installs_all_binaries() {
    let temp_dir = TempDir::new().unwrap();

    create_workspace_cargo_toml(
        temp_dir.path(),
        r#"["crates/tool-a", "crates/tool-b", "crates/my-lib"]"#,
    )
    .unwrap();

    let tool_a_dir = temp_dir.path().join("crates").join("tool-a");
    create_bin_crate(&tool_a_dir, "tool-a").unwrap();

    let tool_b_dir = temp_dir.path().join("crates").join("tool-b");
    create_bin_crate(&tool_b_dir, "tool-b").unwrap();

    let lib_dir = temp_dir.path().join("crates").join("my-lib");
    create_lib_crate(&lib_dir, "my-lib").unwrap();

    // Create target binaries
    let target_dir = temp_dir.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("tool-a"), "fake binary a").unwrap();
    fs::write(target_dir.join("tool-b"), "fake binary b").unwrap();

    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_ok());
    let binaries = result.unwrap().binaries;
    assert_eq!(binaries.len(), 2);
    let names: Vec<&str> = binaries.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"tool-a"));
    assert!(names.contains(&"tool-b"));
}

#[test]
fn test_bin_filter_selects_specific_binary() {
    let temp_dir = TempDir::new().unwrap();

    create_workspace_cargo_toml(temp_dir.path(), r#"["crates/tool-a", "crates/tool-b"]"#).unwrap();

    let tool_a_dir = temp_dir.path().join("crates").join("tool-a");
    create_bin_crate(&tool_a_dir, "tool-a").unwrap();

    let tool_b_dir = temp_dir.path().join("crates").join("tool-b");
    create_bin_crate(&tool_b_dir, "tool-b").unwrap();

    let target_dir = temp_dir.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("tool-a"), "fake binary a").unwrap();
    fs::write(target_dir.join("tool-b"), "fake binary b").unwrap();

    let config = new_config_with_filter(temp_dir.path().to_path_buf(), vec!["tool-b".to_string()]);
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_ok());
    let binaries = result.unwrap().binaries;
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0].0, "tool-b");
}

#[test]
fn test_bin_filter_rejects_unknown_binary() {
    let temp_dir = TempDir::new().unwrap();

    create_workspace_cargo_toml(temp_dir.path(), r#"["crates/tool-a"]"#).unwrap();

    let tool_a_dir = temp_dir.path().join("crates").join("tool-a");
    create_bin_crate(&tool_a_dir, "tool-a").unwrap();

    let config = new_config_with_filter(
        temp_dir.path().to_path_buf(),
        vec!["nonexistent".to_string()],
    );
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::BinaryNotInWorkspace(_)
    ));
}

#[test]
fn test_multi_component_project_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create component workspace
    let cli_component = temp_dir.path().join("components").join("my-cli");
    fs::create_dir_all(&cli_component).unwrap();
    create_workspace_cargo_toml(&cli_component, r#"["crates/cli"]"#).unwrap();

    // Create the binary crate inside the component
    let crate_dir = cli_component.join("crates").join("cli");
    create_bin_crate(&crate_dir, "my-app").unwrap();

    // Create target directory with fake binary
    let target_dir = cli_component.join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("my-app"), "fake binary").unwrap();

    let config = new_config(temp_dir.path().to_path_buf());
    let output = NormalOutput::default();
    let validator = Validator::new(&config, &output);

    let result = validator.validate();
    assert!(result.is_ok());
    let binaries = result.unwrap().binaries;
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0].0, "my-app");
}
