// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::path::PathBuf;
use thiserror::Error;

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
