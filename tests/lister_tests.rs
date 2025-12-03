// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the Lister module.

use serial_test::serial;
use std::fs;
use std::time::SystemTime;
use sw_install::{InstallError, Lister, NormalOutput, SortOrder, format_time_ago};
use tempfile::TempDir;

#[test]
#[serial]
fn test_list_no_binaries() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create empty directory
    fs::create_dir_all(&test_bin_dir).unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 0);
}

#[test]
#[serial]
fn test_list_single_binary() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create directory with one binary
    fs::create_dir_all(&test_bin_dir).unwrap();
    fs::write(test_bin_dir.join("testapp"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 1);
    assert_eq!(binaries[0], "testapp");
}

#[test]
#[serial]
fn test_list_multiple_binaries() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create directory with multiple binaries
    fs::create_dir_all(&test_bin_dir).unwrap();
    fs::write(test_bin_dir.join("app1"), "fake binary").unwrap();
    fs::write(test_bin_dir.join("app2"), "fake binary").unwrap();
    fs::write(test_bin_dir.join("app3"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 3);
    // Should be sorted alphabetically
    assert_eq!(binaries[0], "app1");
    assert_eq!(binaries[1], "app2");
    assert_eq!(binaries[2], "app3");
}

#[test]
#[serial]
fn test_list_ignores_directories() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create directory with binaries and a subdirectory
    fs::create_dir_all(&test_bin_dir).unwrap();
    fs::write(test_bin_dir.join("app1"), "fake binary").unwrap();
    fs::create_dir_all(test_bin_dir.join("subdir")).unwrap();
    fs::write(test_bin_dir.join("app2"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    // Should only include files, not directories
    assert_eq!(binaries.len(), 2);
    assert!(binaries.contains(&"app1".to_string()));
    assert!(binaries.contains(&"app2".to_string()));
}

#[test]
#[serial]
fn test_list_fails_when_dir_does_not_exist() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("nonexistent");

    // Don't create the directory

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        InstallError::InstallDirNotFound(_)
    ));
}

#[test]
#[serial]
fn test_list_sorted_output() {
    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");

    // Create binaries in non-alphabetical order
    fs::create_dir_all(&test_bin_dir).unwrap();
    fs::write(test_bin_dir.join("zebra"), "fake binary").unwrap();
    fs::write(test_bin_dir.join("alpha"), "fake binary").unwrap();
    fs::write(test_bin_dir.join("middle"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 3);
    // Verify alphabetical sorting
    assert_eq!(binaries[0], "alpha");
    assert_eq!(binaries[1], "middle");
    assert_eq!(binaries[2], "zebra");
}

#[test]
#[serial]
fn test_sort_by_oldest() {
    use std::thread;
    use std::time::Duration;

    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");
    fs::create_dir_all(&test_bin_dir).unwrap();

    // Create files with different timestamps
    fs::write(test_bin_dir.join("first"), "fake binary").unwrap();
    thread::sleep(Duration::from_millis(100));
    fs::write(test_bin_dir.join("second"), "fake binary").unwrap();
    thread::sleep(Duration::from_millis(100));
    fs::write(test_bin_dir.join("third"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Oldest, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 3);
    // Should be sorted by modification time, oldest first
    assert_eq!(binaries[0], "first");
    assert_eq!(binaries[1], "second");
    assert_eq!(binaries[2], "third");
}

#[test]
#[serial]
fn test_sort_by_newest() {
    use std::thread;
    use std::time::Duration;

    let temp_home = TempDir::new().unwrap();
    let test_bin_dir = temp_home.path().join("bin");
    fs::create_dir_all(&test_bin_dir).unwrap();

    // Create files with different timestamps
    fs::write(test_bin_dir.join("first"), "fake binary").unwrap();
    thread::sleep(Duration::from_millis(100));
    fs::write(test_bin_dir.join("second"), "fake binary").unwrap();
    thread::sleep(Duration::from_millis(100));
    fs::write(test_bin_dir.join("third"), "fake binary").unwrap();

    let output = NormalOutput::default();
    let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Newest, &output);

    let result = lister.list();
    assert!(result.is_ok());
    let binaries = result.unwrap();
    assert_eq!(binaries.len(), 3);
    // Should be sorted by modification time, newest first
    assert_eq!(binaries[0], "third");
    assert_eq!(binaries[1], "second");
    assert_eq!(binaries[2], "first");
}

#[test]
fn test_format_time_ago_seconds() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(30);
    assert_eq!(format_time_ago(now, then), "30 seconds ago");
}

#[test]
fn test_format_time_ago_minutes() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(120);
    assert_eq!(format_time_ago(now, then), "2 minutes ago");
}

#[test]
fn test_format_time_ago_one_minute() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(60);
    assert_eq!(format_time_ago(now, then), "1 minute ago");
}

#[test]
fn test_format_time_ago_hours() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(3 * 3600);
    assert_eq!(format_time_ago(now, then), "3 hours ago");
}

#[test]
fn test_format_time_ago_days() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(2 * 24 * 3600);
    assert_eq!(format_time_ago(now, then), "2 days ago");
}

#[test]
fn test_format_time_ago_weeks() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(2 * 7 * 24 * 3600);
    assert_eq!(format_time_ago(now, then), "2 weeks ago");
}

#[test]
fn test_format_time_ago_months() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(3 * 30 * 24 * 3600);
    assert_eq!(format_time_ago(now, then), "3 months ago");
}

#[test]
fn test_format_time_ago_years() {
    let now = SystemTime::now();
    let then = now - std::time::Duration::from_secs(2 * 365 * 24 * 3600);
    assert_eq!(format_time_ago(now, then), "2 years ago");
}

#[test]
fn test_sort_order_from_str() {
    assert_eq!("name".parse::<SortOrder>().unwrap(), SortOrder::Name);
    assert_eq!("Name".parse::<SortOrder>().unwrap(), SortOrder::Name);
    assert_eq!("NAME".parse::<SortOrder>().unwrap(), SortOrder::Name);
    assert_eq!("oldest".parse::<SortOrder>().unwrap(), SortOrder::Oldest);
    assert_eq!("newest".parse::<SortOrder>().unwrap(), SortOrder::Newest);
    assert!("invalid".parse::<SortOrder>().is_err());
}
