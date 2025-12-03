// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Installation and uninstallation operations for binaries.

use crate::output::NormalOutput;
use crate::{InstallConfig, InstallError, Result};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// ============================================================================
// Installer
// ============================================================================

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
        self.output.step("[1/3] Creating destination directory...");
        let dest_dir = self.config.destination_dir()?;
        if self.config.test_dir.is_none()
            && !self.config.dry_run
            && let Some(parent) = dest_dir.parent()
            && !parent.exists()
        {
            return Err(InstallError::InstallDirNotFound(dest_dir));
        }
        if !self.config.dry_run {
            fs::create_dir_all(&dest_dir)?;
        }
        self.output
            .info(&format!("Destination: {}", dest_dir.display()));
        self.output.step("[2/3] Copying binary...");
        let dest_binary = self.config.destination_binary_path(&self.binary_name)?;
        if !self.config.dry_run {
            fs::copy(&self.source_binary_path, &dest_binary)?;
        }
        self.output
            .info(&format!("Copied to: {}", dest_binary.display()));
        self.output.step("[3/3] Setting executable permissions...");
        #[cfg(unix)]
        if !self.config.dry_run {
            let mut perms = fs::metadata(&dest_binary)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest_binary, perms)?;
        }
        self.output.success(&format!(
            "Successfully installed: {} -> {}",
            self.binary_name,
            dest_binary.display()
        ));
        Ok(dest_binary)
    }
}

// ============================================================================
// Uninstaller
// ============================================================================

pub struct Uninstaller<'a> {
    binary_name: String,
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a NormalOutput,
}

impl<'a> Uninstaller<'a> {
    pub fn new(
        binary_name: String,
        dry_run: bool,
        test_dir: Option<PathBuf>,
        output: &'a NormalOutput,
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
        let dest_dir = match &self.test_dir {
            Some(dir) => dir.clone(),
            None => {
                let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
                PathBuf::from(home)
                    .join(".local")
                    .join("softwarewrighter")
                    .join("bin")
            }
        };
        let binary_path = dest_dir.join(&self.binary_name);
        self.output
            .info(&format!("Binary path: {}", binary_path.display()));
        self.output.step("[2/2] Validating binary exists...");
        if let Some(parent) = binary_path.parent()
            && !parent.exists()
            && self.test_dir.is_none()
        {
            return Err(InstallError::InstallDirNotFound(parent.to_path_buf()));
        }
        if !binary_path.exists() {
            return Err(InstallError::BinaryNotInstalled(self.binary_name.clone()));
        }
        self.output.step("Removing binary...");
        if !self.dry_run {
            fs::remove_file(&binary_path)?;
        }
        self.output
            .success(&format!("Successfully uninstalled: {}", self.binary_name));
        Ok(())
    }
}
