// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Project validation for sw-install.

mod detect;
mod extract;
mod source;

use std::path::PathBuf;
use sw_install_core::{InstallConfig, InstallError, NormalOutput, Result};

#[derive(Debug)]
pub struct ValidationResult {
    pub binaries: Vec<(String, PathBuf)>,
}

#[derive(Debug)]
pub(crate) enum ProjectType {
    Simple,
    Workspace,
    MultiComponent { component_path: PathBuf },
}

pub struct Validator<'a> {
    pub(crate) config: &'a InstallConfig,
    pub(crate) output: &'a NormalOutput,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: &'a NormalOutput) -> Self {
        Self { config, output }
    }

    pub fn validate(&self) -> Result<ValidationResult> {
        self.output.info("[1/4] Validating project path...");
        self.validate_path()?;
        self.output.info("[2/4] Detecting project structure...");
        let project_type = detect::detect_project_type(self)?;
        self.output.info("[3/4] Extracting binary names...");
        let names = extract::extract_binary_names(self, &project_type)?;
        let filtered = self.apply_bin_filter(names)?;
        self.output
            .info(&format!("Binaries: {}", filtered.join(", ")));
        self.output.info("[4/4] Verifying source binaries exist...");
        let binaries = source::validate_source_binaries(self, &filtered, &project_type)?;
        self.output.success("Validation complete");
        Ok(ValidationResult { binaries })
    }

    fn apply_bin_filter(&self, names: Vec<String>) -> Result<Vec<String>> {
        if self.config.bin_filter.is_empty() {
            return Ok(names);
        }
        for name in &self.config.bin_filter {
            if !names.contains(name) {
                return Err(InstallError::BinaryNotInWorkspace(name.clone()));
            }
        }
        Ok(self.config.bin_filter.clone())
    }

    fn validate_path(&self) -> Result<()> {
        if !self.config.project_path.exists() {
            return Err(InstallError::ProjectNotFound(
                self.config.project_path.clone(),
            ));
        }
        if !self.config.project_path.is_dir() {
            return Err(InstallError::NotADirectory(
                self.config.project_path.clone(),
            ));
        }
        Ok(())
    }
}
