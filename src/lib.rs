// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! sw-install: Binary installer for softwarewrighter CLI tools.

mod lister;
mod operations;
mod output;
mod setup;
mod validator;

use std::path::PathBuf;
use thiserror::Error;

// ============================================================================
// Error types
// ============================================================================

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Project path does not exist: {0}")]
    ProjectNotFound(PathBuf),

    #[error("Project path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("Cargo.toml not found in project: {0}")]
    CargoTomlNotFound(PathBuf),

    #[error("Could not parse Cargo.toml: {0}")]
    CargoTomlParse(String),

    #[error("Binary name not found in Cargo.toml")]
    BinaryNameNotFound,

    #[error(
        "Source binary not found: {0}\nHint: Run 'cargo build --release' in the project directory"
    )]
    BinaryNotFound(PathBuf),

    #[error(
        "Binary is older than source files: {0}\nHint: Run 'cargo build --release' in the project directory"
    )]
    BinaryOutdated(PathBuf),

    #[error("Binary not installed: {0}")]
    BinaryNotInstalled(String),

    #[error(
        "Installation directory does not exist: {0}\nHint: Run 'sw-install --setup-install-dir' to create it and configure PATH"
    )]
    InstallDirNotFound(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid binary name: {0}")]
    InvalidBinaryName(String),

    #[error("Home directory not found")]
    HomeNotFound,

    #[error("No operation specified. Use --project, --uninstall, --list, or --setup-install-dir")]
    NoOperationSpecified,
}

pub type Result<T> = std::result::Result<T, InstallError>;

// ============================================================================
// Configuration
// ============================================================================

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

// ============================================================================
// Re-exports
// ============================================================================

pub use lister::{Lister, SortOrder, format_time_ago};
pub use operations::{Installer, Uninstaller};
pub use output::NormalOutput;
pub use setup::Setup;
pub use validator::Validator;
