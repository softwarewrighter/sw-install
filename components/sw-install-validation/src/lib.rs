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
    pub build_dir: PathBuf,
}

#[derive(Debug)]
pub(crate) enum ProjectType {
    Simple,
    Workspace,
    MultiComponent { component_path: PathBuf },
}

impl ProjectType {
    pub(crate) fn build_dir(&self, project_path: &std::path::Path) -> PathBuf {
        match self {
            Self::Simple | Self::Workspace => project_path.to_path_buf(),
            Self::MultiComponent { component_path } => component_path.clone(),
        }
    }
}

pub struct Validator<'a> {
    pub(crate) config: &'a InstallConfig,
    pub(crate) output: &'a NormalOutput,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: &'a NormalOutput) -> Self {
        Self { config, output }
    }

    pub fn detect_build_dir(&self) -> Result<PathBuf> {
        self.validate_path()?;
        let project_type = detect::detect_project_type(self)?;
        Ok(project_type.build_dir(&self.config.project_path))
    }

    pub fn validate(&self) -> Result<ValidationResult> {
        self.output.info("[1/4] Validating project path...");
        self.validate_path()?;
        self.output.info("[2/4] Detecting project structure...");
        let project_type = detect::detect_project_type(self)?;
        let build_dir = project_type.build_dir(&self.config.project_path);
        self.output.info("[3/4] Extracting binary names...");
        let names = extract::extract_binary_names(self, &project_type)?;
        let filtered = self.apply_bin_filter(names)?;
        self.output
            .info(&format!("Binaries: {}", filtered.join(", ")));
        self.output.info("[4/4] Verifying source binaries exist...");
        let binaries = source::validate_source_binaries(self, &filtered, &project_type)?;
        self.output.success("Validation complete");
        Ok(ValidationResult {
            binaries,
            build_dir,
        })
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
