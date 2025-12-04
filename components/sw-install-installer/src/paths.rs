// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::path::{Path, PathBuf};
use sw_install_core::{InstallError, Result};

pub fn get_dest_dir(test_dir: &Option<PathBuf>) -> Result<PathBuf> {
    match test_dir {
        Some(dir) => Ok(dir.clone()),
        None => {
            let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
            Ok(PathBuf::from(home).join(".local/softwarewrighter/bin"))
        }
    }
}

pub fn validate_binary_exists(path: &Path, name: &str, check_parent: bool) -> Result<PathBuf> {
    if check_parent
        && let Some(parent) = path.parent()
        && !parent.exists()
    {
        return Err(InstallError::InstallDirNotFound(parent.to_path_buf()));
    }
    if !path.exists() {
        return Err(InstallError::BinaryNotInstalled(name.to_string()));
    }
    Ok(path.to_path_buf())
}
