// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::config::InstallConfig;
use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;

#[derive(Debug)]
pub struct ValidationResult {
    pub binary_name: String,
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

        self.output.step("[2/4] Checking Cargo.toml...");
        self.validate_cargo_toml()?;

        self.output.step("[3/4] Extracting binary name...");
        let binary_name = self.extract_binary_name()?;
        self.output.info(&format!("Binary name: {}", binary_name));

        self.output.step("[4/4] Verifying source binary exists...");
        self.validate_source_binary(&binary_name)?;

        self.output.success("Validation complete");

        Ok(ValidationResult { binary_name })
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

    fn validate_cargo_toml(&self) -> Result<()> {
        let cargo_toml_path = self.config.project_path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(InstallError::CargoTomlNotFound(
                self.config.project_path.clone(),
            ));
        }

        Ok(())
    }

    fn extract_binary_name(&self) -> Result<String> {
        let cargo_toml_path = self.config.project_path.join("Cargo.toml");
        let contents = fs::read_to_string(&cargo_toml_path)
            .map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        let value: toml::Value =
            toml::from_str(&contents).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

        // First check for [[bin]] sections
        if let Some(bins) = value.get("bin").and_then(|b| b.as_array()) {
            if let Some(first_bin) = bins.first() {
                if let Some(name) = first_bin.get("name").and_then(|n| n.as_str()) {
                    return Ok(name.to_string());
                }
            }
        }

        // Fall back to package name
        if let Some(package) = value.get("package") {
            if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
                return Ok(name.to_string());
            }
        }

        Err(InstallError::BinaryNameNotFound)
    }

    fn validate_source_binary(&self, binary_name: &str) -> Result<()> {
        let source_path = self.config.source_binary_path(binary_name);
        if !source_path.exists() {
            return Err(InstallError::BinaryNotFound(source_path));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::NormalOutput;
    use std::path::{Path, PathBuf};
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

        let name = validator.extract_binary_name().unwrap();
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

        let name = validator.extract_binary_name().unwrap();
        assert_eq!(name, "my-binary");
    }
}
