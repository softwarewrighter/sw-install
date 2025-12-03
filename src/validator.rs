// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::output::NormalOutput;
use crate::{InstallConfig, InstallError, Result};
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

pub struct Validator<'a> {
    config: &'a InstallConfig,
    output: &'a NormalOutput,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: &'a NormalOutput) -> Self {
        Self { config, output }
    }

    pub fn validate(&self) -> Result<ValidationResult> {
        self.output.step("[1/4] Validating project path...");
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
        self.output.step("[2/4] Detecting project structure...");
        let project_type = self.detect_project_type()?;
        let type_name = match &project_type {
            ProjectType::Simple => "simple package",
            ProjectType::Workspace => "workspace",
            ProjectType::MultiComponent { .. } => "multi-component",
        };
        self.output.info(&format!("Project type: {}", type_name));
        self.output.step("[3/4] Extracting binary name...");
        let cargo_toml_path = match &project_type {
            ProjectType::Simple | ProjectType::Workspace => {
                self.config.project_path.join("Cargo.toml")
            }
            ProjectType::MultiComponent { component_path } => component_path.join("Cargo.toml"),
        };
        let binary_name = self.extract_binary_name(&cargo_toml_path)?;
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

    fn detect_project_type(&self) -> Result<ProjectType> {
        let cargo_toml_path = self.config.project_path.join("Cargo.toml");
        if cargo_toml_path.exists()
            && let Ok(contents) = fs::read_to_string(&cargo_toml_path)
            && let Ok(value) = toml::from_str::<toml::Value>(&contents)
        {
            if value.get("workspace").is_some() {
                return Ok(ProjectType::Workspace);
            }
            if value.get("package").is_some() {
                return Ok(ProjectType::Simple);
            }
        }
        let components_dir = self.config.project_path.join("components");
        if let Ok(entries) = fs::read_dir(&components_dir) {
            for component_path in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
                if let Ok(contents) = fs::read_to_string(component_path.join("Cargo.toml"))
                    && let Ok(value) = toml::from_str::<toml::Value>(&contents)
                    && let Some(workspace) = value.get("workspace")
                    && let Some(members) = workspace.get("members").and_then(|m| m.as_array())
                    && !self
                        .find_workspace_binaries_at(&component_path, members)
                        .is_empty()
                {
                    return Ok(ProjectType::MultiComponent { component_path });
                }
            }
        }
        Err(InstallError::CargoTomlNotFound(
            self.config.project_path.clone(),
        ))
    }

    fn extract_binary_name(&self, cargo_toml_path: &Path) -> Result<String> {
        let contents = fs::read_to_string(cargo_toml_path)
            .map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;
        let value: toml::Value =
            toml::from_str(&contents).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;
        if let Some(workspace) = value.get("workspace")
            && let Some(members) = workspace.get("members").and_then(|m| m.as_array())
        {
            let binaries = self.find_workspace_binaries_at(
                cargo_toml_path.parent().unwrap_or(Path::new(".")),
                members,
            );
            if !binaries.is_empty() {
                if binaries.len() > 1 {
                    self.output
                        .info(&format!("Multiple binaries found: {}", binaries.join(", ")));
                }
                return Ok(binaries.into_iter().next().unwrap());
            }
        }
        if let Some(bins) = value.get("bin").and_then(|b| b.as_array())
            && let Some(first_bin) = bins.first()
            && let Some(name) = first_bin.get("name").and_then(|n| n.as_str())
        {
            return Ok(name.to_string());
        }
        if let Some(package) = value.get("package")
            && let Some(name) = package.get("name").and_then(|n| n.as_str())
        {
            return Ok(name.to_string());
        }
        Err(InstallError::BinaryNameNotFound)
    }

    fn find_workspace_binaries_at(&self, root: &Path, members: &[toml::Value]) -> Vec<String> {
        let mut binaries = Vec::new();
        for member in members.iter().filter_map(|m| m.as_str()) {
            let paths: Vec<PathBuf> = if let Some(base) = member.strip_suffix("/*") {
                fs::read_dir(root.join(base))
                    .map(|e| {
                        e.filter_map(|e| e.ok())
                            .filter(|e| e.path().is_dir())
                            .map(|e| PathBuf::from(base).join(e.file_name()))
                            .collect()
                    })
                    .unwrap_or_default()
            } else {
                vec![PathBuf::from(member)]
            };
            for path in paths {
                if let Ok(contents) = fs::read_to_string(root.join(&path).join("Cargo.toml"))
                    && let Ok(value) = toml::from_str::<toml::Value>(&contents)
                {
                    if let Some(bins) = value.get("bin").and_then(|b| b.as_array()) {
                        binaries.extend(
                            bins.iter()
                                .filter_map(|b| b.get("name").and_then(|n| n.as_str()))
                                .map(String::from),
                        );
                    } else if let Some(pkg) = value.get("package")
                        && let Some(name) = pkg.get("name").and_then(|n| n.as_str())
                        && root.join(&path).join("src/main.rs").exists()
                    {
                        binaries.push(name.to_string());
                    }
                }
            }
        }
        binaries
    }

    fn validate_source_binary_for_type(
        &self,
        binary_name: &str,
        project_type: &ProjectType,
    ) -> Result<PathBuf> {
        let (source_path, source_root) = match project_type {
            ProjectType::Simple | ProjectType::Workspace => (
                self.config.source_binary_path(binary_name),
                self.config.project_path.clone(),
            ),
            ProjectType::MultiComponent { component_path } => (
                component_path
                    .join("target")
                    .join(self.config.target_subdir())
                    .join(binary_name),
                component_path.clone(),
            ),
        };
        if !source_path.exists() {
            return Err(InstallError::BinaryNotFound(source_path));
        }
        let binary_time = fs::metadata(&source_path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        if let Some(source_time) = find_newest_source_file(&source_root)
            && source_time > binary_time
        {
            return Err(InstallError::BinaryOutdated(source_path));
        }
        Ok(source_path)
    }
}

/// Recursively find the newest .rs file modification time
fn find_newest_source_file(dir: &Path) -> Option<SystemTime> {
    let mut newest: Option<SystemTime> = None;
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
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
    newest
}
