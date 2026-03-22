// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::shell::{find_shell_config, write_path_config};
use std::fs;
use std::path::{Path, PathBuf};
use sw_install_core::{InstallError, NormalOutput, Result};

pub struct Setup<'a> {
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a NormalOutput,
}

impl<'a> Setup<'a> {
    pub fn new(dry_run: bool, test_dir: Option<PathBuf>, output: &'a NormalOutput) -> Self {
        Self {
            dry_run,
            test_dir,
            output,
        }
    }

    pub fn setup(&self) -> Result<()> {
        self.output.info("[1/3] Creating installation directory...");
        let install_dir = self.create_install_dir()?;
        self.output
            .info(&format!("Created: {}", install_dir.display()));
        self.output.info("[2/3] Detecting shell configuration...");
        let shell_config = self.configure_shell(&install_dir)?;
        self.output.success(&format!(
            "\nSetup complete!\n\nInstallation directory: {}\n\nTo activate PATH changes, run:\n  source {}",
            install_dir.display(), shell_config.display()
        ));
        Ok(())
    }

    pub(crate) fn create_install_dir(&self) -> Result<PathBuf> {
        let install_dir = self.test_dir.clone().map_or_else(
            || {
                Ok(
                    PathBuf::from(std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?)
                        .join(".local/softwarewrighter/bin"),
                )
            },
            Ok::<_, InstallError>,
        )?;
        if !self.dry_run {
            fs::create_dir_all(&install_dir)?;
        }
        Ok(install_dir)
    }

    pub(crate) fn configure_shell(&self, install_dir: &Path) -> Result<PathBuf> {
        let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        let shell_config = find_shell_config(Path::new(&home));
        self.output
            .info(&format!("Shell config: {}", shell_config.display()));
        self.output
            .info("[3/3] Adding PATH configuration to shell...");
        if self.test_dir.is_some() {
            self.output.info("Test mode: skipping shell configuration");
            return Ok(shell_config);
        }
        write_path_config(&shell_config, install_dir, self.dry_run, self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use sw_install_core::NormalOutput;
    use tempfile::TempDir;

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
        assert!(!test_path.exists());
    }

    #[test]
    #[serial]
    fn test_setup_creates_directory() {
        let temp_home = TempDir::new().unwrap();
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
}
