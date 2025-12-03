// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::output::NormalOutput;
use crate::{InstallError, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
            self.output.info("Test mode: skipping shell configuration");
            return Ok(());
        }
        let path_line = format!("export PATH=\"{}:$PATH\"", install_dir.display());
        if self.dry_run {
            self.output.info(&format!(
                "Would add to {}: {}",
                shell_config.display(),
                path_line
            ));
            return Ok(());
        }
        let content = fs::read_to_string(shell_config).unwrap_or_default();
        if content.contains(&path_line) {
            self.output.info("PATH already configured in shell config");
            return Ok(());
        }
        let sep = if content.is_empty() || content.ends_with('\n') {
            ""
        } else {
            "\n"
        };
        let new_content = format!("{}{}\n# Added by sw-install\n{}\n", content, sep, path_line);
        fs::write(shell_config, new_content)?;
        Ok(())
    }
}
