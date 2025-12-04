// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::{ProjectType, Validator};
use std::fs;
use std::path::Path;
use sw_install_core::{InstallError, Result};

pub(crate) fn extract_binary_name(
    validator: &Validator,
    project_type: &ProjectType,
) -> Result<String> {
    let cargo_toml = get_cargo_toml_path(validator, project_type);
    let contents =
        fs::read_to_string(&cargo_toml).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;
    let value: toml::Value =
        toml::from_str(&contents).map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;
    try_extract_from_workspace(validator, &cargo_toml, &value)
        .or_else(|| try_extract_from_bin(&value))
        .or_else(|| try_extract_from_package(&value))
        .ok_or(InstallError::BinaryNameNotFound)
}

fn get_cargo_toml_path(validator: &Validator, project_type: &ProjectType) -> std::path::PathBuf {
    match project_type {
        ProjectType::Simple | ProjectType::Workspace => {
            validator.config.project_path.join("Cargo.toml")
        }
        ProjectType::MultiComponent { component_path } => component_path.join("Cargo.toml"),
    }
}

fn try_extract_from_workspace(
    validator: &Validator,
    cargo_toml: &Path,
    value: &toml::Value,
) -> Option<String> {
    let ws = value.get("workspace")?;
    let members = ws.get("members").and_then(|m| m.as_array())?;
    let binaries = sw_install_workspace::find_workspace_binaries(cargo_toml.parent()?, members);
    if binaries.is_empty() {
        return None;
    }
    if binaries.len() > 1 {
        validator
            .output
            .info(&format!("Multiple binaries: {}", binaries.join(", ")));
    }
    binaries.into_iter().next()
}

fn try_extract_from_bin(value: &toml::Value) -> Option<String> {
    let bins = value.get("bin").and_then(|b| b.as_array())?;
    let first = bins.first()?;
    first.get("name").and_then(|n| n.as_str()).map(String::from)
}

fn try_extract_from_package(value: &toml::Value) -> Option<String> {
    let pkg = value.get("package")?;
    pkg.get("name").and_then(|n| n.as_str()).map(String::from)
}
