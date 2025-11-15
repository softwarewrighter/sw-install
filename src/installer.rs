// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::config::InstallConfig;
use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct Installer<'a> {
    config: &'a InstallConfig,
    binary_name: String,
    output: &'a dyn OutputHandler,
}

impl<'a> Installer<'a> {
    pub fn new(
        config: &'a InstallConfig,
        binary_name: String,
        output: &'a dyn OutputHandler,
    ) -> Self {
        Self {
            config,
            binary_name,
            output,
        }
    }

    pub fn install(&self) -> Result<PathBuf> {
        self.output.step("[1/3] Creating destination directory...");
        let dest_dir = self.create_destination_dir()?;
        self.output
            .info(&format!("Destination: {}", dest_dir.display()));

        self.output.step("[2/3] Copying binary...");
        let dest_binary = self.copy_binary(&dest_dir)?;
        self.output
            .info(&format!("Copied to: {}", dest_binary.display()));

        self.output.step("[3/3] Setting executable permissions...");
        self.set_permissions(&dest_binary)?;

        self.output.success(&format!(
            "Successfully installed: {} -> {}",
            self.binary_name,
            dest_binary.display()
        ));

        Ok(dest_binary)
    }

    fn create_destination_dir(&self) -> Result<PathBuf> {
        let dest_dir = self.config.destination_dir()?;

        // Check if parent directory exists (unless using test_dir)
        if self.config.test_dir.is_none() && !self.config.dry_run {
            let parent = dest_dir
                .parent()
                .ok_or_else(|| InstallError::InstallDirNotFound(dest_dir.clone()))?;

            if !parent.exists() {
                return Err(InstallError::InstallDirNotFound(dest_dir.clone()));
            }
        }

        if !self.config.dry_run {
            fs::create_dir_all(&dest_dir)?;
        }

        Ok(dest_dir)
    }

    fn copy_binary(&self, _dest_dir: &Path) -> Result<PathBuf> {
        let source = self.config.source_binary_path(&self.binary_name);
        let dest = self.config.destination_binary_path(&self.binary_name)?;

        if !self.config.dry_run {
            fs::copy(&source, &dest)?;
        }

        Ok(dest)
    }

    fn set_permissions(&self, binary_path: &Path) -> Result<()> {
        if !self.config.dry_run {
            #[cfg(unix)]
            {
                let mut perms = fs::metadata(binary_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(binary_path, perms)?;
            }
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
        let output = NormalOutput;
        let installer = Installer::new(&config, "testapp".to_string(), &output);

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
        fs::write(target_dir.join("testapp"), source_content).unwrap();

        let config = InstallConfig::new(
            temp_project.path().to_path_buf(),
            None,
            false,
            false,
            false,
            Some(test_bin_dir.clone()),
        );
        let output = NormalOutput;
        let installer = Installer::new(&config, "testapp".to_string(), &output);

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
        fs::write(target_dir.join("testapp"), "fake binary").unwrap();

        let config = InstallConfig::new(
            temp_project.path().to_path_buf(),
            Some("testapp-dev".to_string()),
            false,
            false,
            false,
            Some(test_bin_dir.clone()),
        );
        let output = NormalOutput;
        let installer = Installer::new(&config, "testapp".to_string(), &output);

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
        fs::write(target_dir.join("testapp"), "fake binary").unwrap();

        let config = InstallConfig::new(
            temp_project.path().to_path_buf(),
            None,
            false,
            false,
            true, // dry_run = true
            Some(test_bin_dir.clone()),
        );
        let output = NormalOutput;
        let installer = Installer::new(&config, "testapp".to_string(), &output);

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
        fs::write(target_dir.join("testapp"), "fake binary").unwrap();

        let config = InstallConfig::new(
            temp_project.path().to_path_buf(),
            None,
            false,
            false,
            false,
            Some(test_bin_dir.clone()),
        );
        let output = NormalOutput;
        let installer = Installer::new(&config, "testapp".to_string(), &output);

        let dest_path = installer.install().unwrap();
        let metadata = fs::metadata(&dest_path).unwrap();
        let permissions = metadata.permissions();

        // Check that executable bit is set
        assert_eq!(permissions.mode() & 0o111, 0o111);
    }
}
