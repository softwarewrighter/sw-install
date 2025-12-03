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
    source_binary_path: PathBuf,
    output: &'a dyn OutputHandler,
}

impl<'a> Installer<'a> {
    pub fn new(
        config: &'a InstallConfig,
        binary_name: String,
        source_binary_path: PathBuf,
        output: &'a dyn OutputHandler,
    ) -> Self {
        Self {
            config,
            binary_name,
            source_binary_path,
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
        let dest = self.config.destination_binary_path(&self.binary_name)?;

        if !self.config.dry_run {
            fs::copy(&self.source_binary_path, &dest)?;
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
