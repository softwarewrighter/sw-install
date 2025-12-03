// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::error::{InstallError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub project_path: PathBuf,
    pub binary_name: Option<String>,
    pub use_debug: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub test_dir: Option<PathBuf>,
}

impl InstallConfig {
    pub fn new(
        project_path: PathBuf,
        binary_name: Option<String>,
        use_debug: bool,
        verbose: bool,
        dry_run: bool,
        test_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            project_path,
            binary_name,
            use_debug,
            verbose,
            dry_run,
            test_dir,
        }
    }

    /// Get the destination directory for installed binaries
    pub fn destination_dir(&self) -> Result<PathBuf> {
        if let Some(ref test_dir) = self.test_dir {
            return Ok(test_dir.clone());
        }

        let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("softwarewrighter")
            .join("bin"))
    }

    /// Get the path to the source binary
    pub fn source_binary_path(&self, actual_name: &str) -> PathBuf {
        self.project_path
            .join("target")
            .join(self.target_subdir())
            .join(actual_name)
    }

    /// Get the path to the destination binary
    pub fn destination_binary_path(&self, actual_name: &str) -> Result<PathBuf> {
        let dest_dir = self.destination_dir()?;
        let final_name = self.binary_name.as_deref().unwrap_or(actual_name);
        Ok(dest_dir.join(final_name))
    }

    /// Get the target subdirectory (debug or release)
    pub fn target_subdir(&self) -> &str {
        if self.use_debug { "debug" } else { "release" }
    }
}
