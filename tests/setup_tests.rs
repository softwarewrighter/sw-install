// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the Setup module.

use serial_test::serial;
use std::fs;
use sw_install::{NormalOutput, Setup};
use tempfile::TempDir;

#[test]
#[serial]
fn test_setup_creates_directory() {
    let temp_home = TempDir::new().unwrap();
    // SAFETY: This test runs serially and no other threads access HOME
    unsafe { std::env::set_var("HOME", temp_home.path()) };

    let output = NormalOutput::default();
    let setup = Setup::new(false, None, &output);

    let result = setup.create_install_dir();
    assert!(result.is_ok());

    let install_dir = result.unwrap();
    assert!(install_dir.exists());
    assert!(
        install_dir
            .to_string_lossy()
            .ends_with(".local/softwarewrighter/bin")
    );
}

#[test]
fn test_setup_with_test_dir() {
    let test_dir = TempDir::new().unwrap();
    let test_path = test_dir.path().join("custom-bin");

    let output = NormalOutput::default();
    let setup = Setup::new(false, Some(test_path.clone()), &output);

    let result = setup.create_install_dir();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), test_path);
    assert!(test_path.exists());
}

#[test]
fn test_setup_dry_run() {
    let test_dir = TempDir::new().unwrap();
    let test_path = test_dir.path().join("custom-bin");

    let output = NormalOutput::default();
    let setup = Setup::new(true, Some(test_path.clone()), &output);

    let result = setup.create_install_dir();
    assert!(result.is_ok());
    // Directory should not be created in dry-run mode
    assert!(!test_path.exists());
}

#[test]
#[serial]
fn test_detect_shell_config_finds_zshrc() {
    let temp_home = TempDir::new().unwrap();
    // SAFETY: This test runs serially and no other threads access HOME
    unsafe { std::env::set_var("HOME", temp_home.path()) };

    let zshrc = temp_home.path().join(".zshrc");
    fs::write(&zshrc, "# test config").unwrap();

    let output = NormalOutput::default();
    let setup = Setup::new(false, None, &output);

    let result = setup.detect_shell_config();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), zshrc);
}

#[test]
#[serial]
fn test_detect_shell_config_defaults_to_bashrc() {
    let temp_home = TempDir::new().unwrap();
    // SAFETY: This test runs serially and no other threads access HOME
    unsafe { std::env::set_var("HOME", temp_home.path()) };

    let output = NormalOutput::default();
    let setup = Setup::new(false, None, &output);

    let result = setup.detect_shell_config();
    assert!(result.is_ok());
    assert!(result.unwrap().to_string_lossy().ends_with(".bashrc"));
}

#[test]
fn test_configure_path_in_test_mode() {
    let test_dir = TempDir::new().unwrap();
    let shell_config = test_dir.path().join(".bashrc");
    let install_dir = test_dir.path().join("bin");

    let output = NormalOutput::default();
    let setup = Setup::new(false, Some(install_dir.clone()), &output);

    let result = setup.configure_path(&shell_config, &install_dir);
    assert!(result.is_ok());
    // Shell config should not be created/modified in test mode
    assert!(!shell_config.exists());
}

#[test]
fn test_configure_path_dry_run() {
    let test_dir = TempDir::new().unwrap();
    let shell_config = test_dir.path().join(".bashrc");
    let install_dir = test_dir.path().join("bin");
    fs::write(&shell_config, "# existing config\n").unwrap();

    let output = NormalOutput::default();
    let setup = Setup::new(true, None, &output);

    let result = setup.configure_path(&shell_config, &install_dir);
    assert!(result.is_ok());

    // File should not be modified in dry-run
    let content = fs::read_to_string(&shell_config).unwrap();
    assert_eq!(content, "# existing config\n");
}

#[test]
#[serial]
fn test_full_setup_with_test_dir() {
    let test_dir = TempDir::new().unwrap();
    let install_dir = test_dir.path().join("bin");

    let output = NormalOutput::default();
    let setup = Setup::new(false, Some(install_dir.clone()), &output);

    let result = setup.setup();
    assert!(result.is_ok());
    assert!(install_dir.exists());
}
