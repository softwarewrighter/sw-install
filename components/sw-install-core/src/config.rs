// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::{InstallError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub project_path: PathBuf,
    pub rename: Option<String>,
    pub bin_filter: Vec<String>,
    pub use_debug: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub build: bool,
    pub test_dir: Option<PathBuf>,
}

impl InstallConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_path: PathBuf,
        rename: Option<String>,
        bin_filter: Vec<String>,
        use_debug: bool,
        verbose: bool,
        dry_run: bool,
        build: bool,
        test_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            project_path,
            rename,
            bin_filter,
            use_debug,
            verbose,
            dry_run,
            build,
            test_dir,
        }
    }

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

    pub fn source_binary_path(&self, actual_name: &str) -> PathBuf {
        let subdir = if self.use_debug { "debug" } else { "release" };
        self.project_path
            .join("target")
            .join(subdir)
            .join(actual_name)
    }
}
