// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::config::InstallConfig;
use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::{Path, PathBuf};

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

        Ok(source_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::NormalOutput;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_project(dir: &Path, include_binary: bool) -> std::io::Result<()> {
        let cargo_toml = dir.join("Cargo.toml");
        fs::write(
            cargo_toml,
            r#"[package]
name = "test-app"
version = "0.1.0"
edition = "2021"
"#,
        )?;

        if include_binary {
            let target_dir = dir.join("target").join("release");
            fs::create_dir_all(&target_dir)?;
            fs::write(target_dir.join("test-app"), "fake binary")?;
        }

        Ok(())
    }

    fn create_workspace_cargo_toml(dir: &Path, members: &str) -> std::io::Result<()> {
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                r#"[workspace]
resolver = "2"
members = {members}
"#
            ),
        )
    }

    fn create_package_cargo_toml(dir: &Path, name: &str, bin: Option<&str>) -> std::io::Result<()> {
        let bin_section = bin
            .map(|n| {
                format!(
                    r#"
[[bin]]
name = "{n}"
path = "src/main.rs"
"#
                )
            })
            .unwrap_or_default();
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                r#"[package]
name = "{name}"
version = "0.1.0"
{bin_section}"#
            ),
        )
    }

    fn create_lib_crate(dir: &Path, name: &str) -> std::io::Result<()> {
        fs::create_dir_all(dir.join("src"))?;
        create_package_cargo_toml(dir, name, None)?;
        fs::write(dir.join("src").join("lib.rs"), "pub fn foo() {}")
    }

    fn create_bin_crate(dir: &Path, name: &str) -> std::io::Result<()> {
        fs::create_dir_all(dir.join("src"))?;
        create_package_cargo_toml(dir, name, Some(name))?;
        fs::write(dir.join("src").join("main.rs"), "fn main() {}")
    }

    #[test]
    fn test_validate_fails_when_project_path_missing() {
        let config = InstallConfig::new(
            PathBuf::from("/nonexistent"),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let result = validator.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InstallError::ProjectNotFound(_)
        ));
    }

    #[test]
    fn test_validate_fails_when_cargo_toml_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let result = validator.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InstallError::CargoTomlNotFound(_)
        ));
    }

    #[test]
    fn test_validate_fails_when_binary_missing() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), false).unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let result = validator.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InstallError::BinaryNotFound(_)
        ));
    }

    #[test]
    fn test_validate_succeeds_with_valid_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), true).unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let result = validator.validate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().binary_name, "test-app");
    }

    #[test]
    fn test_extract_binary_name_from_package() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(
            cargo_toml,
            r#"[package]
name = "my-binary"
version = "0.1.0"
"#,
        )
        .unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let name = validator.extract_binary_from_simple().unwrap();
        assert_eq!(name, "my-binary");
    }

    #[test]
    fn test_extract_binary_name_from_bin_section() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(
            cargo_toml,
            r#"[package]
name = "my-package"
version = "0.1.0"

[[bin]]
name = "my-binary"
path = "src/main.rs"
"#,
        )
        .unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let name = validator.extract_binary_from_simple().unwrap();
        assert_eq!(name, "my-binary");
    }

    #[test]
    fn test_extract_binary_name_from_workspace_with_bin_section() {
        let temp_dir = TempDir::new().unwrap();

        // Create workspace root Cargo.toml
        let root_cargo = temp_dir.path().join("Cargo.toml");
        fs::write(
            root_cargo,
            r#"[workspace]
resolver = "2"
members = ["crates/my-cli"]
"#,
        )
        .unwrap();

        // Create member crate directory and Cargo.toml
        let crate_dir = temp_dir.path().join("crates").join("my-cli");
        fs::create_dir_all(&crate_dir).unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            r#"[package]
name = "my-cli"
version = "0.1.0"

[[bin]]
name = "my-cli"
path = "src/main.rs"
"#,
        )
        .unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let name = validator
            .extract_binary_from_workspace(temp_dir.path())
            .unwrap();
        assert_eq!(name, "my-cli");
    }

    #[test]
    fn test_extract_binary_name_from_workspace_with_main_rs() {
        let temp_dir = TempDir::new().unwrap();

        // Create workspace root Cargo.toml
        let root_cargo = temp_dir.path().join("Cargo.toml");
        fs::write(
            root_cargo,
            r#"[workspace]
resolver = "2"
members = ["crates/my-app"]
"#,
        )
        .unwrap();

        // Create member crate with src/main.rs (default binary)
        let crate_dir = temp_dir.path().join("crates").join("my-app");
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            r#"[package]
name = "my-app"
version = "0.1.0"
"#,
        )
        .unwrap();
        fs::write(crate_dir.join("src").join("main.rs"), "fn main() {}").unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let name = validator
            .extract_binary_from_workspace(temp_dir.path())
            .unwrap();
        assert_eq!(name, "my-app");
    }

    #[test]
    fn test_workspace_with_library_only_members_ignores_libs() {
        let temp_dir = TempDir::new().unwrap();

        create_workspace_cargo_toml(temp_dir.path(), r#"["crates/my-lib", "crates/my-cli"]"#)
            .unwrap();

        let lib_dir = temp_dir.path().join("crates").join("my-lib");
        create_lib_crate(&lib_dir, "my-lib").unwrap();

        let cli_dir = temp_dir.path().join("crates").join("my-cli");
        create_bin_crate(&cli_dir, "my-cli").unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let name = validator
            .extract_binary_from_workspace(temp_dir.path())
            .unwrap();
        assert_eq!(name, "my-cli");
    }

    #[test]
    fn test_multi_component_project_detection() {
        let temp_dir = TempDir::new().unwrap();

        // Create component workspace
        let cli_component = temp_dir.path().join("components").join("my-cli");
        fs::create_dir_all(&cli_component).unwrap();
        create_workspace_cargo_toml(&cli_component, r#"["crates/cli"]"#).unwrap();

        // Create the binary crate inside the component
        let crate_dir = cli_component.join("crates").join("cli");
        create_bin_crate(&crate_dir, "my-app").unwrap();

        // Create target directory with fake binary
        let target_dir = cli_component.join("target").join("release");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("my-app"), "fake binary").unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None,
            false,
            false,
            false,
            None,
        );
        let output = NormalOutput;
        let validator = Validator::new(&config, &output);

        let result = validator.validate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().binary_name, "my-app");
    }
}
