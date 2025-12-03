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

    pub fn create_install_dir(&self) -> Result<PathBuf> {
        let install_dir = self.destination_dir()?;

        if !self.dry_run {
            fs::create_dir_all(&install_dir)?;
        }

        Ok(install_dir)
    }

    pub fn detect_shell_config(&self) -> Result<PathBuf> {
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

    pub fn configure_path(&self, shell_config: &Path, install_dir: &Path) -> Result<()> {
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
