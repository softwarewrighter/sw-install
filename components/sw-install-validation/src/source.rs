// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::{ProjectType, Validator};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use sw_install_core::{InstallError, Result};

pub(crate) fn validate_source_binary(
    validator: &Validator,
    binary_name: &str,
    project_type: &ProjectType,
) -> Result<PathBuf> {
    let (source_path, source_root) = get_source_paths(validator, binary_name, project_type);
    if !source_path.exists() {
        return Err(InstallError::BinaryNotFound(source_path.to_path_buf()));
    }
    check_freshness(&source_path, &source_root)?;
    Ok(source_path)
}

fn get_source_paths(
    validator: &Validator,
    binary_name: &str,
    project_type: &ProjectType,
) -> (PathBuf, PathBuf) {
    match project_type {
        ProjectType::Simple | ProjectType::Workspace => (
            validator.config.source_binary_path(binary_name),
            validator.config.project_path.clone(),
        ),
        ProjectType::MultiComponent { component_path } => {
            let subdir = if validator.config.use_debug {
                "debug"
            } else {
                "release"
            };
            (
                component_path.join("target").join(subdir).join(binary_name),
                component_path.clone(),
            )
        }
    }
}

fn check_freshness(source_path: &Path, source_root: &Path) -> Result<()> {
    let binary_time = fs::metadata(source_path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);
    if let Some(source_time) = find_newest_source_file(source_root)
        && source_time > binary_time
    {
        return Err(InstallError::BinaryOutdated(source_path.to_path_buf()));
    }
    Ok(())
}

fn find_newest_source_file(dir: &Path) -> Option<SystemTime> {
    let entries = fs::read_dir(dir).ok()?;
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() != "target")
        .fold(None, |newest, entry| {
            let time = get_entry_time(&entry.path());
            match (newest, time) {
                (Some(n), Some(t)) => Some(n.max(t)),
                (None, t) => t,
                (n, None) => n,
            }
        })
}

fn get_entry_time(path: &Path) -> Option<SystemTime> {
    if path.is_dir() {
        find_newest_source_file(path)
    } else if path.extension().is_some_and(|e| e == "rs") {
        fs::metadata(path).and_then(|m| m.modified()).ok()
    } else {
        None
    }
}
