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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = InstallConfig::new(
            PathBuf::from("/test/path"),
            Some("renamed".to_string()),
            true,
            true,
            false,
            None,
        );

        assert_eq!(config.project_path, PathBuf::from("/test/path"));
        assert_eq!(config.binary_name, Some("renamed".to_string()));
        assert!(config.use_debug);
        assert!(config.verbose);
        assert!(!config.dry_run);
        assert!(config.test_dir.is_none());
    }

    #[test]
    fn test_destination_dir() {
        let config = InstallConfig::new(PathBuf::from("/test"), None, false, false, false, None);

        let dest = config.destination_dir().unwrap();
        assert!(
            dest.to_string_lossy()
                .ends_with(".local/softwarewrighter/bin")
        );
    }

    #[test]
    fn test_destination_dir_with_test_dir() {
        let config = InstallConfig::new(
            PathBuf::from("/test"),
            None,
            false,
            false,
            false,
            Some(PathBuf::from("/custom/test/dir")),
        );

        let dest = config.destination_dir().unwrap();
        assert_eq!(dest, PathBuf::from("/custom/test/dir"));
    }

    #[test]
    fn test_source_binary_path_release() {
        let config = InstallConfig::new(
            PathBuf::from("/test/project"),
            None,
            false,
            false,
            false,
            None,
        );

        let source = config.source_binary_path("myapp");
        assert_eq!(source, PathBuf::from("/test/project/target/release/myapp"));
    }

    #[test]
    fn test_source_binary_path_debug() {
        let config = InstallConfig::new(
            PathBuf::from("/test/project"),
            None,
            true,
            false,
            false,
            None,
        );

        let source = config.source_binary_path("myapp");
        assert_eq!(source, PathBuf::from("/test/project/target/debug/myapp"));
    }

    #[test]
    fn test_destination_binary_path_no_rename() {
        let config = InstallConfig::new(
            PathBuf::from("/test/project"),
            None,
            false,
            false,
            false,
            None,
        );

        let dest = config.destination_binary_path("myapp").unwrap();
        assert!(
            dest.to_string_lossy()
                .ends_with("softwarewrighter/bin/myapp")
        );
    }

    #[test]
    fn test_destination_binary_path_with_rename() {
        let config = InstallConfig::new(
            PathBuf::from("/test/project"),
            Some("myapp-dev".to_string()),
            false,
            false,
            false,
            None,
        );

        let dest = config.destination_binary_path("myapp").unwrap();
        assert!(
            dest.to_string_lossy()
                .ends_with("softwarewrighter/bin/myapp-dev")
        );
    }

    #[test]
    fn test_target_subdir_release() {
        let config = InstallConfig::new(PathBuf::from("/test"), None, false, false, false, None);

        assert_eq!(config.target_subdir(), "release");
    }

    #[test]
    fn test_target_subdir_debug() {
        let config = InstallConfig::new(PathBuf::from("/test"), None, true, false, false, None);

        assert_eq!(config.target_subdir(), "debug");
    }
}
