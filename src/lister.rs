// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::PathBuf;

pub struct Lister<'a> {
    test_dir: Option<PathBuf>,
    output: &'a dyn OutputHandler,
}

impl<'a> Lister<'a> {
    pub fn new(test_dir: Option<PathBuf>, output: &'a dyn OutputHandler) -> Self {
        Self { test_dir, output }
    }

    pub fn list(&self) -> Result<Vec<String>> {
        self.output.step("Listing installed binaries...");

        let bin_dir = self.destination_dir()?;

        // Check if directory exists
        if !bin_dir.exists() {
            return Err(InstallError::InstallDirNotFound(bin_dir));
        }

        // Read directory entries
        let entries = fs::read_dir(&bin_dir)?;
        let mut binaries = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Only include files (not directories or symlinks)
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        binaries.push(name_str.to_string());
                    }
                }
            }
        }

        binaries.sort();

        // Always show list output (not just in verbose mode)
        if binaries.is_empty() {
            println!("No binaries installed");
        } else {
            for binary in &binaries {
                println!("{}", binary);
            }
        }

        Ok(binaries)
    }

    fn destination_dir(&self) -> Result<PathBuf> {
        if let Some(ref test_dir) = self.test_dir {
            return Ok(test_dir.clone());
        }

        let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("softwarewrighter")
            .join("bin"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::NormalOutput;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_list_no_binaries() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create empty directory
        fs::create_dir_all(&test_bin_dir).unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

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

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

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

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

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

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

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

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

        let result = lister.list();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InstallError::InstallDirNotFound(_)
        ));
    }

    #[test]
    #[serial]
    fn test_destination_dir_with_test_dir() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

        let dest = lister.destination_dir().unwrap();
        assert_eq!(dest, test_bin_dir);
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

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 3);
        // Verify alphabetical sorting
        assert_eq!(binaries[0], "alpha");
        assert_eq!(binaries[1], "middle");
        assert_eq!(binaries[2], "zebra");
    }
}
