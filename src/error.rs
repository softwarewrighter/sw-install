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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_project_not_found() {
        let error = InstallError::ProjectNotFound(PathBuf::from("/foo/bar"));
        assert_eq!(error.to_string(), "Project path does not exist: /foo/bar");
    }

    #[test]
    fn test_error_display_cargo_toml_not_found() {
        let error = InstallError::CargoTomlNotFound(PathBuf::from("/foo/bar"));
        assert_eq!(
            error.to_string(),
            "Cargo.toml not found in project: /foo/bar"
        );
    }

    #[test]
    fn test_error_display_binary_not_found() {
        let error = InstallError::BinaryNotFound(PathBuf::from("/foo/bar/target/release/app"));
        let message = error.to_string();
        assert!(message.contains("Source binary not found"));
        assert!(message.contains("/foo/bar/target/release/app"));
        assert!(message.contains("cargo build --release"));
    }

    #[test]
    fn test_error_display_home_not_found() {
        let error = InstallError::HomeNotFound;
        assert_eq!(error.to_string(), "Home directory not found");
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error: InstallError = io_error.into();
        assert!(error.to_string().contains("IO error"));
    }

    #[test]
    fn test_error_no_operation_specified_mentions_all_operations() {
        let error = InstallError::NoOperationSpecified;
        let message = error.to_string();

        // Verify the error message mentions all available operations
        assert!(
            message.contains("--project"),
            "Error should mention --project"
        );
        assert!(
            message.contains("--uninstall"),
            "Error should mention --uninstall"
        );
        assert!(message.contains("--list"), "Error should mention --list");
        assert!(
            message.contains("--setup-install-dir"),
            "Error should mention --setup-install-dir"
        );
    }
}
