// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::fs;
use std::path::{Path, PathBuf};
use sw_install_core::{InstallConfig, InstallError, NormalOutput, Result};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct Installer<'a> {
    config: &'a InstallConfig,
    binary_name: String,
    source_binary_path: PathBuf,
    output: &'a NormalOutput,
}

impl<'a> Installer<'a> {
    pub fn new(
        config: &'a InstallConfig,
        binary_name: String,
        source_binary_path: PathBuf,
        output: &'a NormalOutput,
    ) -> Self {
        Self {
            config,
            binary_name,
            source_binary_path,
            output,
        }
    }

    pub fn install(&self) -> Result<PathBuf> {
        let dest_dir = self.prepare_destination()?;
        let dest_binary = self.copy_and_set_permissions(&dest_dir)?;
        self.output.success(&format!(
            "Successfully installed: {} -> {}",
            self.binary_name,
            dest_binary.display()
        ));
        Ok(dest_binary)
    }

    fn prepare_destination(&self) -> Result<PathBuf> {
        self.output.info("[1/3] Creating destination directory...");
        let dest_dir = self.config.destination_dir()?;
        if self.config.test_dir.is_none()
            && !self.config.dry_run
            && let Some(parent) = dest_dir.parent()
            && !parent.exists()
        {
            return Err(InstallError::InstallDirNotFound(dest_dir.clone()));
        }
        if !self.config.dry_run {
            fs::create_dir_all(&dest_dir)?;
        }
        self.output
            .info(&format!("Destination: {}", dest_dir.display()));
        Ok(dest_dir)
    }

    fn copy_and_set_permissions(&self, dest_dir: &Path) -> Result<PathBuf> {
        self.output.info("[2/3] Copying binary...");
        let final_name = self
            .config
            .binary_name
            .as_deref()
            .unwrap_or(&self.binary_name);
        let dest_binary = dest_dir.join(final_name);
        if !self.config.dry_run {
            fs::copy(&self.source_binary_path, &dest_binary)?;
        }
        self.output
            .info(&format!("Copied to: {}", dest_binary.display()));
        self.output.info("[3/3] Setting executable permissions...");
        #[cfg(unix)]
        if !self.config.dry_run {
            let mut perms = fs::metadata(&dest_binary)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest_binary, perms)?;
        }
        Ok(dest_binary)
    }
}
