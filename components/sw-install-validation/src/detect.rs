// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::{ProjectType, Validator};
use std::fs;
use sw_install_core::{InstallError, Result};

pub(crate) fn detect_project_type(validator: &Validator) -> Result<ProjectType> {
    if let Some(pt) = try_detect_from_cargo_toml(validator) {
        return Ok(pt);
    }
    if let Some(pt) = try_detect_multi_component(validator) {
        return Ok(pt);
    }
    Err(InstallError::CargoTomlNotFound(
        validator.config.project_path.clone(),
    ))
}

fn try_detect_from_cargo_toml(validator: &Validator) -> Option<ProjectType> {
    let cargo_toml = validator.config.project_path.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml).ok()?;
    let value: toml::Value = toml::from_str(&contents).ok()?;
    if value.get("workspace").is_some() {
        validator.output.info("Project type: workspace");
        return Some(ProjectType::Workspace);
    }
    if value.get("package").is_some() {
        validator.output.info("Project type: simple package");
        return Some(ProjectType::Simple);
    }
    None
}

fn try_detect_multi_component(validator: &Validator) -> Option<ProjectType> {
    let components = validator.config.project_path.join("components");
    let entries = fs::read_dir(&components).ok()?;
    for path in entries.filter_map(|e| e.ok()).map(|e| e.path()) {
        if is_valid_component(&path) {
            validator.output.info("Project type: multi-component");
            return Some(ProjectType::MultiComponent {
                component_path: path,
            });
        }
    }
    None
}

fn is_valid_component(path: &std::path::Path) -> bool {
    let Ok(contents) = fs::read_to_string(path.join("Cargo.toml")) else {
        return false;
    };
    let Ok(value) = toml::from_str::<toml::Value>(&contents) else {
        return false;
    };
    // Check for workspace with binaries
    if let Some(ws) = value.get("workspace")
        && let Some(members) = ws.get("members").and_then(|m| m.as_array())
    {
        return !sw_install_workspace::find_workspace_binaries(path, members).is_empty();
    }
    // Check for simple package with binary
    value.get("package").is_some() && value.get("bin").is_some()
}
