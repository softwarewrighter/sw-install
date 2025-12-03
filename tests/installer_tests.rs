// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the Installer module.

use serial_test::serial;
use std::fs;
use sw_install::{InstallConfig, Installer, NormalOutput};
use tempfile::TempDir;

#[test]
#[serial]
fn test_install_creates_directory() {
    let temp_project = TempDir::new().unwrap();
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create source binary
    let target_dir = temp_project.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(target_dir.join("testapp"), "fake binary").unwrap();

    let config = InstallConfig::new(
        temp_project.path().to_path_buf(),
        None,
        false,
        false,
        false,
        Some(test_bin_dir.clone()),
    );
    let output = NormalOutput::default();
    let source_path = target_dir.join("testapp");
    let installer = Installer::new(&config, "testapp".to_string(), source_path, &output);

    let result = installer.install();
    assert!(result.is_ok());
    assert!(test_bin_dir.exists());
}

#[test]
#[serial]
fn test_install_copies_binary() {
    let temp_project = TempDir::new().unwrap();
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create source binary
    let target_dir = temp_project.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    let source_content = b"fake binary content";
    let source_path = target_dir.join("testapp");
    fs::write(&source_path, source_content).unwrap();

    let config = InstallConfig::new(
        temp_project.path().to_path_buf(),
        None,
        false,
        false,
        false,
        Some(test_bin_dir.clone()),
    );
    let output = NormalOutput::default();
    let installer = Installer::new(&config, "testapp".to_string(), source_path, &output);

    let dest_path = installer.install().unwrap();
    assert!(dest_path.exists());

    let dest_content = fs::read(&dest_path).unwrap();
    assert_eq!(dest_content, source_content);
}

#[test]
#[serial]
fn test_install_with_rename() {
    let temp_project = TempDir::new().unwrap();
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create source binary
    let target_dir = temp_project.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    let source_path = target_dir.join("testapp");
    fs::write(&source_path, "fake binary").unwrap();

    let config = InstallConfig::new(
        temp_project.path().to_path_buf(),
        Some("testapp-dev".to_string()),
        false,
        false,
        false,
        Some(test_bin_dir.clone()),
    );
    let output = NormalOutput::default();
    let installer = Installer::new(&config, "testapp".to_string(), source_path, &output);

    let dest_path = installer.install().unwrap();
    assert!(dest_path.to_string_lossy().ends_with("testapp-dev"));
    assert!(dest_path.exists());
}

#[test]
#[serial]
fn test_dry_run_doesnt_modify_filesystem() {
    let temp_project = TempDir::new().unwrap();
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create source binary
    let target_dir = temp_project.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    let source_path = target_dir.join("testapp");
    fs::write(&source_path, "fake binary").unwrap();

    let config = InstallConfig::new(
        temp_project.path().to_path_buf(),
        None,
        false,
        false,
        true, // dry_run = true
        Some(test_bin_dir.clone()),
    );
    let output = NormalOutput::default();
    let installer = Installer::new(&config, "testapp".to_string(), source_path, &output);

    let result = installer.install();
    assert!(result.is_ok());

    // Verify destination directory was NOT created
    assert!(!test_bin_dir.exists());
}

#[cfg(unix)]
#[test]
#[serial]
fn test_sets_executable_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_project = TempDir::new().unwrap();
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create source binary
    let target_dir = temp_project.path().join("target").join("release");
    fs::create_dir_all(&target_dir).unwrap();
    let source_path = target_dir.join("testapp");
    fs::write(&source_path, "fake binary").unwrap();

    let config = InstallConfig::new(
        temp_project.path().to_path_buf(),
        None,
        false,
        false,
        false,
        Some(test_bin_dir.clone()),
    );
    let output = NormalOutput::default();
    let installer = Installer::new(&config, "testapp".to_string(), source_path, &output);

    let dest_path = installer.install().unwrap();
    let metadata = fs::metadata(&dest_path).unwrap();
    let permissions = metadata.permissions();

    // Check that executable bit is set
    assert_eq!(permissions.mode() & 0o111, 0o111);
}
