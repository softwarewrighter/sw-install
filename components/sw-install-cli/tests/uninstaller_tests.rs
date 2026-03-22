// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the Uninstaller module.

use serial_test::serial;
use std::fs;
use sw_install::{InstallError, NormalOutput, Uninstaller};
use tempfile::TempDir;

#[test]
#[serial]
fn test_uninstall_removes_binary() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create installed binary
    fs::create_dir_all(&test_bin_dir).unwrap();
    let binary_path = test_bin_dir.join("testapp");
    fs::write(&binary_path, "fake binary").unwrap();

    let output = NormalOutput::default();
    let uninstaller = Uninstaller::new(
        "testapp".to_string(),
        false,
        Some(test_bin_dir.clone()),
        &output,
    );

    let result = uninstaller.uninstall();
    assert!(result.is_ok());
    assert!(!binary_path.exists());
}

#[test]
#[serial]
fn test_uninstall_fails_when_binary_not_installed() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create the directory but not the binary
    fs::create_dir_all(&test_bin_dir).unwrap();

    let output = NormalOutput::default();
    let uninstaller = Uninstaller::new(
        "nonexistent".to_string(),
        false,
        Some(test_bin_dir.clone()),
        &output,
    );

    let result = uninstaller.uninstall();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::BinaryNotInstalled(_)
    ));
}

#[test]
#[serial]
fn test_uninstall_dry_run_doesnt_remove() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create installed binary
    fs::create_dir_all(&test_bin_dir).unwrap();
    let binary_path = test_bin_dir.join("testapp");
    fs::write(&binary_path, "fake binary").unwrap();

    let output = NormalOutput::default();
    let uninstaller = Uninstaller::new(
        "testapp".to_string(),
        true,
        Some(test_bin_dir.clone()),
        &output,
    ); // dry_run = true

    let result = uninstaller.uninstall();
    assert!(result.is_ok());
    assert!(binary_path.exists()); // Binary should still exist
}
