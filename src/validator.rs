// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::config::InstallConfig;
use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug)]
pub struct ValidationResult {
    pub binary_name: String,
    pub source_binary_path: PathBuf,
}

/// Represents the different project structures we support
#[derive(Debug)]
enum ProjectType {
    /// Simple project with Cargo.toml containing [package]
    Simple,
    /// Workspace project with Cargo.toml containing [workspace]
    Workspace,
    /// Multi-component project with components/<name>/Cargo.toml workspaces
    MultiComponent {
        /// Path to the component workspace that contains the binary
        component_path: PathBuf,
    },
}

impl ProjectType {
    fn description(&self) -> &'static str {
        match self {
            ProjectType::Simple => "simple package",
            ProjectType::Workspace => "workspace",
            ProjectType::MultiComponent { .. } => "multi-component",
        }
    }
}

pub struct Validator<'a> {
    config: &'a InstallConfig,
    output: &'a dyn OutputHandler,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: &'a dyn OutputHandler) -> Self {
        Self { config, output }
    }

    pub fn validate(&self) -> Result<ValidationResult> {
        self.output.step("[1/4] Validating project path...");
        self.validate_project_path()?;

        self.output.step("[2/4] Detecting project structure...");
        let project_type = self.detect_project_type()?;
        self.output
            .info(&format!("Project type: {}", project_type.description()));

        self.output.step("[3/4] Extracting binary name...");
        let binary_name = self.extract_binary_name_for_type(&project_type)?;
        self.output.info(&format!("Binary name: {}", binary_name));

        self.output.step("[4/4] Verifying source binary exists...");
        let source_binary_path =
            self.validate_source_binary_for_type(&binary_name, &project_type)?;

        self.output.success("Validation complete");

        Ok(ValidationResult {
            binary_name,
            source_binary_path,
        })
    }

    fn validate_project_path(&self) -> Result<()> {
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

    /// Detect the project structure type
    fn detect_project_type(&self) -> Result<ProjectType> {
        let cargo_toml_path = self.config.project_path.join("Cargo.toml");

        // Check if root Cargo.toml exists
        if cargo_toml_path.exists()
            && let Ok(contents) = fs::read_to_string(&cargo_toml_path)
            && let Ok(value) = toml::from_str::<toml::Value>(&contents)
        {
            // Check if it's a workspace
            if value.get("workspace").is_some() {
                return Ok(ProjectType::Workspace);
            }
            // It has a [package] section - simple project
            if value.get("package").is_some() {
                return Ok(ProjectType::Simple);
            }
        }

        // No root Cargo.toml - check for components/ directory
        let components_dir = self.config.project_path.join("components");
        if components_dir.is_dir()
            && let Some(component_path) = self.find_component_with_binary(&components_dir)
        {
            return Ok(ProjectType::MultiComponent { component_path });
        }

        // No valid project structure found
        Err(InstallError::CargoTomlNotFound(
            self.config.project_path.clone(),
        ))
    }

    /// Find a component directory that contains a binary crate
    fn find_component_with_binary(&self, components_dir: &Path) -> Option<PathBuf> {
        if let Ok(entries) = fs::read_dir(components_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let component_path = entry.path();
                if component_path.is_dir() {
                    let cargo_toml = component_path.join("Cargo.toml");
                    if cargo_toml.exists() {
                        // Check if this workspace contains binaries
                        if let Ok(contents) = fs::read_to_string(&cargo_toml)
                            && let Ok(value) = toml::from_str::<toml::Value>(&contents)
                            && let Some(workspace) = value.get("workspace")
                            && let Some(members) =
                                workspace.get("members").and_then(|m| m.as_array())
                        {
                            let binaries =
                                self.find_workspace_binaries_at(&component_path, members);
                            if !binaries.is_empty() {
                                return Some(component_path);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_binary_name_for_type(&self, project_type: &ProjectType) -> Result<String> {
        match project_type {
            ProjectType::Simple => self.extract_binary_from_simple(),
            ProjectType::Workspace => self.extract_binary_from_workspace(&self.config.project_path),
            ProjectType::MultiComponent { component_path } => {
                self.extract_binary_from_workspace(component_path)
            }
        }
    }

    fn extract_binary_from_simple(&self) -> Result<String> {
        let cargo_toml_path = self.config.project_path.join("Cargo.toml");
        let contents = fs::read_to_string(&cargo_toml_path)
            .map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        let value: toml::Value =
            toml::from_str(&contents).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        // First check for [[bin]] sections
        if let Some(bins) = value.get("bin").and_then(|b| b.as_array())
            && let Some(first_bin) = bins.first()
            && let Some(name) = first_bin.get("name").and_then(|n| n.as_str())
        {
            return Ok(name.to_string());
        }

        // Fall back to package name
        if let Some(package) = value.get("package")
            && let Some(name) = package.get("name").and_then(|n| n.as_str())
        {
            return Ok(name.to_string());
        }

        Err(InstallError::BinaryNameNotFound)
    }

    fn extract_binary_from_workspace(&self, workspace_root: &Path) -> Result<String> {
        let cargo_toml_path = workspace_root.join("Cargo.toml");
        let contents = fs::read_to_string(&cargo_toml_path)
            .map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        let value: toml::Value =
            toml::from_str(&contents).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        if let Some(workspace) = value.get("workspace")
            && let Some(members) = workspace.get("members").and_then(|m| m.as_array())
        {
            let binary_names = self.find_workspace_binaries_at(workspace_root, members);
            if binary_names.len() == 1 {
                return Ok(binary_names.into_iter().next().unwrap());
            } else if binary_names.len() > 1 {
                self.output.info(&format!(
                    "Multiple binaries found in workspace: {}",
                    binary_names.join(", ")
                ));
                return Ok(binary_names.into_iter().next().unwrap());
            }
        }

        Err(InstallError::BinaryNameNotFound)
    }

    /// Scan workspace members to find binary crates at a given workspace root
    fn find_workspace_binaries_at(
        &self,
        workspace_root: &Path,
        members: &[toml::Value],
    ) -> Vec<String> {
        let mut binary_names = Vec::new();

        for member in members {
            if let Some(member_path) = member.as_str() {
                // Handle glob patterns (e.g., "crates/*")
                let member_paths = self.expand_workspace_member_at(workspace_root, member_path);

                for path in member_paths {
                    let member_cargo_toml = workspace_root.join(&path).join("Cargo.toml");
                    if member_cargo_toml.exists()
                        && let Ok(contents) = fs::read_to_string(&member_cargo_toml)
                        && let Ok(value) = toml::from_str::<toml::Value>(&contents)
                    {
                        // Check for [[bin]] sections
                        if let Some(bins) = value.get("bin").and_then(|b| b.as_array()) {
                            for bin in bins {
                                if let Some(name) = bin.get("name").and_then(|n| n.as_str()) {
                                    binary_names.push(name.to_string());
                                }
                            }
                        }
                        // Also check if package has default binary (has src/main.rs)
                        else if let Some(package) = value.get("package")
                            && let Some(name) = package.get("name").and_then(|n| n.as_str())
                        {
                            let main_rs = workspace_root.join(&path).join("src").join("main.rs");
                            if main_rs.exists() {
                                binary_names.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        binary_names
    }

    /// Expand workspace member patterns at a given workspace root
    fn expand_workspace_member_at(&self, workspace_root: &Path, pattern: &str) -> Vec<PathBuf> {
        // Simple glob expansion - handle patterns ending with /*
        if let Some(base) = pattern.strip_suffix("/*") {
            let base_path = workspace_root.join(base);
            if let Ok(entries) = fs::read_dir(&base_path) {
                return entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| PathBuf::from(base).join(e.file_name()))
                    .collect();
            }
            Vec::new()
        } else {
            vec![PathBuf::from(pattern)]
        }
    }

    fn validate_source_binary_for_type(
        &self,
        binary_name: &str,
        project_type: &ProjectType,
    ) -> Result<PathBuf> {
        let source_path = match project_type {
            ProjectType::Simple | ProjectType::Workspace => {
                self.config.source_binary_path(binary_name)
            }
            ProjectType::MultiComponent { component_path } => component_path
                .join("target")
                .join(self.config.target_subdir())
                .join(binary_name),
        };

        if !source_path.exists() {
            return Err(InstallError::BinaryNotFound(source_path));
        }

        // Check if binary is older than source files
        let source_root = match project_type {
            ProjectType::Simple | ProjectType::Workspace => self.config.project_path.clone(),
            ProjectType::MultiComponent { component_path } => component_path.clone(),
        };
        self.check_binary_freshness(&source_path, &source_root)?;

        Ok(source_path)
    }

    /// Check if the binary is newer than all source files
    fn check_binary_freshness(&self, binary_path: &Path, source_root: &Path) -> Result<()> {
        let binary_time = fs::metadata(binary_path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let newest_source = find_newest_source_file(source_root);

        if let Some(source_time) = newest_source
            && source_time > binary_time
        {
            return Err(InstallError::BinaryOutdated(binary_path.to_path_buf()));
        }

        Ok(())
    }
}

/// Recursively find the newest .rs file modification time
fn find_newest_source_file(dir: &Path) -> Option<SystemTime> {
    let mut newest: Option<SystemTime> = None;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip target directory
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }

            if path.is_dir() {
                if let Some(time) = find_newest_source_file(&path) {
                    newest = Some(newest.map_or(time, |n| n.max(time)));
                }
            } else if path.extension().is_some_and(|e| e == "rs")
                && let Ok(metadata) = fs::metadata(&path)
                && let Ok(modified) = metadata.modified()
            {
                newest = Some(newest.map_or(modified, |n| n.max(modified)));
            }
        }
    }

    newest
}
