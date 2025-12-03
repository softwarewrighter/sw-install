// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Uninstaller<'a> {
    binary_name: String,
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a dyn OutputHandler,
}

impl<'a> Uninstaller<'a> {
    pub fn new(
        binary_name: String,
        dry_run: bool,
        test_dir: Option<PathBuf>,
        output: &'a dyn OutputHandler,
    ) -> Self {
        Self {
            binary_name,
            dry_run,
            test_dir,
            output,
        }
    }

    pub fn uninstall(&self) -> Result<()> {
        self.output.step("[1/2] Locating binary...");
        let binary_path = self.binary_path()?;
        self.output
            .info(&format!("Binary path: {}", binary_path.display()));

        self.output.step("[2/2] Validating binary exists...");
        self.validate_binary_exists(&binary_path)?;

        self.output.step("Removing binary...");
        self.remove_binary(&binary_path)?;

        self.output
            .success(&format!("Successfully uninstalled: {}", self.binary_name));

        Ok(())
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

    fn binary_path(&self) -> Result<PathBuf> {
        let dest_dir = self.destination_dir()?;
        Ok(dest_dir.join(&self.binary_name))
    }

    fn validate_binary_exists(&self, binary_path: &Path) -> Result<()> {
        // Check if the installation directory itself exists
        if let Some(parent) = binary_path.parent()
            && !parent.exists()
            && self.test_dir.is_none()
        {
            return Err(InstallError::InstallDirNotFound(parent.to_path_buf()));
        }

        if !binary_path.exists() {
            return Err(InstallError::BinaryNotInstalled(self.binary_name.clone()));
        }
        Ok(())
    }

    fn remove_binary(&self, binary_path: &Path) -> Result<()> {
        if !self.dry_run {
            fs::remove_file(binary_path)?;
        }
        Ok(())
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
    fn test_uninstall_removes_binary() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create installed binary
        fs::create_dir_all(&test_bin_dir).unwrap();
        let binary_path = test_bin_dir.join("testapp");
        fs::write(&binary_path, "fake binary").unwrap();

        let output = NormalOutput;
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

        let output = NormalOutput;
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

        let output = NormalOutput;
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

    #[test]
    #[serial]
    fn test_binary_path_construction() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        let output = NormalOutput;
        let uninstaller = Uninstaller::new(
            "myapp".to_string(),
            false,
            Some(test_bin_dir.clone()),
            &output,
        );

        let path = uninstaller.binary_path().unwrap();
        assert_eq!(path, test_bin_dir.join("myapp"));
    }
}
