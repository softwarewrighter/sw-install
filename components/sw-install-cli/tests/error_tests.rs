// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the InstallError module.

use std::path::PathBuf;
use sw_install::InstallError;

#[test]
fn test_error_display_project_not_found() {
    let error = InstallError::ProjectNotFound(PathBuf::from("/foo/bar"));
    assert_eq!(error.to_string(), "Project path does not exist: /foo/bar");
}

#[test]
fn test_error_display_cargo_toml_not_found() {
    let error = InstallError::CargoTomlNotFound(PathBuf::from("/foo/bar"));
    assert_eq!(
        error.to_string(),
        "Cargo.toml not found in project: /foo/bar"
    );
}

#[test]
fn test_error_display_binary_not_found() {
    let error = InstallError::BinaryNotFound(PathBuf::from("/foo/bar/target/release/app"));
    let message = error.to_string();
    assert!(message.contains("Source binary not found"));
    assert!(message.contains("/foo/bar/target/release/app"));
    assert!(message.contains("cargo build --release"));
}

#[test]
fn test_error_display_home_not_found() {
    let error = InstallError::HomeNotFound;
    assert_eq!(error.to_string(), "Home directory not found");
}

#[test]
fn test_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: InstallError = io_error.into();
    assert!(error.to_string().contains("IO error"));
}

#[test]
fn test_error_no_operation_specified_mentions_all_operations() {
    let error = InstallError::NoOperationSpecified;
    let message = error.to_string();

    assert!(
        message.contains("--project"),
        "Error should mention --project"
    );
    assert!(
        message.contains("--uninstall"),
        "Error should mention --uninstall"
    );
    assert!(message.contains("--list"), "Error should mention --list");
    assert!(
        message.contains("--setup-install-dir"),
        "Error should mention --setup-install-dir"
    );
}
