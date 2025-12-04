// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the InstallConfig module.

use std::path::PathBuf;
use sw_install::InstallConfig;

#[test]
fn test_new_config() {
    let config = InstallConfig::new(
        PathBuf::from("/test/path"),
        Some("renamed".to_string()),
        true,
        true,
        false,
        None,
    );

    assert_eq!(config.project_path, PathBuf::from("/test/path"));
    assert_eq!(config.binary_name, Some("renamed".to_string()));
    assert!(config.use_debug);
    assert!(config.verbose);
    assert!(!config.dry_run);
    assert!(config.test_dir.is_none());
}

#[test]
fn test_destination_dir() {
    let config = InstallConfig::new(PathBuf::from("/test"), None, false, false, false, None);

    let dest = config.destination_dir().unwrap();
    assert!(
        dest.to_string_lossy()
            .ends_with(".local/softwarewrighter/bin")
    );
}

#[test]
fn test_destination_dir_with_test_dir() {
    let config = InstallConfig::new(
        PathBuf::from("/test"),
        None,
        false,
        false,
        false,
        Some(PathBuf::from("/custom/test/dir")),
    );

    let dest = config.destination_dir().unwrap();
    assert_eq!(dest, PathBuf::from("/custom/test/dir"));
}

#[test]
fn test_source_binary_path_release() {
    let config = InstallConfig::new(
        PathBuf::from("/test/project"),
        None,
        false,
        false,
        false,
        None,
    );

    let source = config.source_binary_path("myapp");
    assert_eq!(source, PathBuf::from("/test/project/target/release/myapp"));
}

#[test]
fn test_source_binary_path_debug() {
    let config = InstallConfig::new(
        PathBuf::from("/test/project"),
        None,
        true,
        false,
        false,
        None,
    );

    let source = config.source_binary_path("myapp");
    assert_eq!(source, PathBuf::from("/test/project/target/debug/myapp"));
}
