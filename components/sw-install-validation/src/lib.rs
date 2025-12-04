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
    pub binary_name: String,
    pub source_binary_path: PathBuf,
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
        self.output.info("[3/4] Extracting binary name...");
        let binary_name = extract::extract_binary_name(self, &project_type)?;
        self.output.info(&format!("Binary name: {}", binary_name));
        self.output.info("[4/4] Verifying source binary exists...");
        let source_binary_path = source::validate_source_binary(self, &binary_name, &project_type)?;
        self.output.success("Validation complete");
        Ok(ValidationResult {
            binary_name,
            source_binary_path,
        })
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
