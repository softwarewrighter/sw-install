// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::paths::{get_dest_dir, validate_binary_exists};
use std::fs;
use std::path::PathBuf;
use sw_install_core::{NormalOutput, Result};

pub struct Uninstaller<'a> {
    binary_name: String,
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a NormalOutput,
}

impl<'a> Uninstaller<'a> {
    pub fn new(
        name: String,
        dry_run: bool,
        test_dir: Option<PathBuf>,
        out: &'a NormalOutput,
    ) -> Self {
        Self {
            binary_name: name,
            dry_run,
            test_dir,
            output: out,
        }
    }

    pub fn uninstall(&self) -> Result<()> {
        let binary_path = self.locate_and_validate()?;
        self.remove_binary(&binary_path)?;
        self.output
            .success(&format!("Successfully uninstalled: {}", self.binary_name));
        Ok(())
    }

    fn locate_and_validate(&self) -> Result<PathBuf> {
        self.output.info("[1/2] Locating binary...");
        let dest_dir = get_dest_dir(&self.test_dir)?;
        let binary_path = dest_dir.join(&self.binary_name);
        self.output
            .info(&format!("Binary path: {}", binary_path.display()));
        self.output.info("[2/2] Validating binary exists...");
        validate_binary_exists(&binary_path, &self.binary_name, self.test_dir.is_none())
    }

    fn remove_binary(&self, binary_path: &PathBuf) -> Result<()> {
        self.output.info("Removing binary...");
        if !self.dry_run {
            fs::remove_file(binary_path)?;
        }
        Ok(())
    }
}
