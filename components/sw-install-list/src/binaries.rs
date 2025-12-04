// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use sw_install_core::{InstallError, Result};

pub fn get_bin_dir(test_dir: &Option<PathBuf>) -> Result<PathBuf> {
    let bin_dir = match test_dir {
        Some(dir) => dir.clone(),
        None => {
            let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
            PathBuf::from(home).join(".local/softwarewrighter/bin")
        }
    };
    if !bin_dir.exists() {
        return Err(InstallError::InstallDirNotFound(bin_dir));
    }
    Ok(bin_dir)
}

pub fn collect_binaries(bin_dir: &PathBuf) -> Result<Vec<(String, SystemTime)>> {
    Ok(fs::read_dir(bin_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_string();
            let time = fs::metadata(e.path()).and_then(|m| m.modified()).ok()?;
            Some((name, time))
        })
        .collect())
}
