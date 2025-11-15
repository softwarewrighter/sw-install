// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Setup<'a> {
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a dyn OutputHandler,
}

impl<'a> Setup<'a> {
    pub fn new(dry_run: bool, test_dir: Option<PathBuf>, output: &'a dyn OutputHandler) -> Self {
        Self {
            dry_run,
            test_dir,
            output,
        }
    }

    pub fn setup(&self) -> Result<()> {
        self.output.step("[1/3] Creating installation directory...");
        let install_dir = self.create_install_dir()?;
        self.output
            .info(&format!("Created: {}", install_dir.display()));

        self.output.step("[2/3] Detecting shell configuration...");
        let shell_config = self.detect_shell_config()?;
        self.output
            .info(&format!("Shell config: {}", shell_config.display()));

        self.output
            .step("[3/3] Adding PATH configuration to shell...");
        self.configure_path(&shell_config, &install_dir)?;

        self.output.success(&format!(
            "\nSetup complete!\n\nInstallation directory: {}\n\nTo activate PATH changes, run:\n  source {}",
            install_dir.display(),
            shell_config.display()
        ));

        Ok(())
    }

    fn destination_dir(&self) -> Result<PathBuf> {
        if let Some(ref test_dir) = self.test_dir {
            return Ok(test_dir.clone());
        }

        let home = env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("softwarewrighter")
            .join("bin"))
    }

    fn create_install_dir(&self) -> Result<PathBuf> {
        let install_dir = self.destination_dir()?;

        if !self.dry_run {
            fs::create_dir_all(&install_dir)?;
        }

        Ok(install_dir)
    }

    fn detect_shell_config(&self) -> Result<PathBuf> {
        let home = env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        let home_path = Path::new(&home);

        // Check for common shell config files in order of preference
        let configs = vec![
            home_path.join(".zshrc"),
            home_path.join(".bashrc"),
            home_path.join(".bash_profile"),
            home_path.join(".profile"),
        ];

        for config in configs {
            if config.exists() {
                return Ok(config);
            }
        }

        // Default to .bashrc if nothing exists
        Ok(home_path.join(".bashrc"))
    }

    fn configure_path(&self, shell_config: &Path, install_dir: &Path) -> Result<()> {
        if self.test_dir.is_some() {
            // In test mode, don't actually modify shell config
            self.output.info("Test mode: skipping shell configuration");
            return Ok(());
        }

        if self.dry_run {
            self.output.info(&format!(
                "Would add to {}: export PATH=\"{}:$PATH\"",
                shell_config.display(),
                install_dir.display()
            ));
            return Ok(());
        }

        // Read existing config
        let content = if shell_config.exists() {
            fs::read_to_string(shell_config)?
        } else {
            String::new()
        };

        // Check if already configured
        let path_line = format!("export PATH=\"{}:$PATH\"", install_dir.display());
        if content.contains(&path_line) {
            self.output.info("PATH already configured in shell config");
            return Ok(());
        }

        // Append PATH configuration
        let mut new_content = content;
        if !new_content.is_empty() && !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push('\n');
        new_content.push_str("# Added by sw-install\n");
        new_content.push_str(&path_line);
        new_content.push('\n');

        fs::write(shell_config, new_content)?;

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
    fn test_setup_creates_directory() {
        let temp_home = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_home.path());

        let output = NormalOutput;
        let setup = Setup::new(false, None, &output);

        let result = setup.create_install_dir();
        assert!(result.is_ok());

        let install_dir = result.unwrap();
        assert!(install_dir.exists());
        assert!(install_dir
            .to_string_lossy()
            .ends_with(".local/softwarewrighter/bin"));
    }

    #[test]
    fn test_setup_with_test_dir() {
        let test_dir = TempDir::new().unwrap();
        let test_path = test_dir.path().join("custom-bin");

        let output = NormalOutput;
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

        let output = NormalOutput;
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
        std::env::set_var("HOME", temp_home.path());

        let zshrc = temp_home.path().join(".zshrc");
        fs::write(&zshrc, "# test config").unwrap();

        let output = NormalOutput;
        let setup = Setup::new(false, None, &output);

        let result = setup.detect_shell_config();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), zshrc);
    }

    #[test]
    #[serial]
    fn test_detect_shell_config_defaults_to_bashrc() {
        let temp_home = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_home.path());

        let output = NormalOutput;
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

        let output = NormalOutput;
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

        let output = NormalOutput;
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

        let output = NormalOutput;
        let setup = Setup::new(false, Some(install_dir.clone()), &output);

        let result = setup.setup();
        assert!(result.is_ok());
        assert!(install_dir.exists());
    }
}
