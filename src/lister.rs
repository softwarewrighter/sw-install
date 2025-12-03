// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::error::{InstallError, Result};
use crate::output::OutputHandler;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name,
    Oldest,
    Newest,
}

#[derive(Debug, Clone)]
pub struct InvalidSortOrder(String);

impl std::fmt::Display for InvalidSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid sort order '{}'. Valid options are: name, oldest, newest",
            self.0
        )
    }
}

impl std::error::Error for InvalidSortOrder {}

impl FromStr for SortOrder {
    type Err = InvalidSortOrder;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(SortOrder::Name),
            "oldest" => Ok(SortOrder::Oldest),
            "newest" => Ok(SortOrder::Newest),
            _ => Err(InvalidSortOrder(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct BinaryInfo {
    name: String,
    modified_time: SystemTime,
}

pub struct Lister<'a> {
    test_dir: Option<PathBuf>,
    sort_order: SortOrder,
    output: &'a dyn OutputHandler,
}

impl<'a> Lister<'a> {
    pub fn new(
        test_dir: Option<PathBuf>,
        sort_order: SortOrder,
        output: &'a dyn OutputHandler,
    ) -> Self {
        Self {
            test_dir,
            sort_order,
            output,
        }
    }

    pub fn list(&self) -> Result<Vec<String>> {
        self.output.step("Listing installed binaries...");

        let bin_dir = self.destination_dir()?;
        if !bin_dir.exists() {
            return Err(InstallError::InstallDirNotFound(bin_dir));
        }

        let mut binaries = self.collect_binaries(&bin_dir)?;
        self.sort_binaries(&mut binaries);
        self.print_binaries(&binaries);

        Ok(binaries.iter().map(|b| b.name.clone()).collect())
    }

    fn collect_binaries(&self, bin_dir: &Path) -> Result<Vec<BinaryInfo>> {
        let entries = fs::read_dir(bin_dir)?;
        let mut binaries = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && let Some(name) = path.file_name()
                && let Some(name_str) = name.to_str()
            {
                let metadata = fs::metadata(&path)?;
                let modified_time = metadata.modified()?;
                binaries.push(BinaryInfo {
                    name: name_str.to_string(),
                    modified_time,
                });
            }
        }
        Ok(binaries)
    }

    fn sort_binaries(&self, binaries: &mut [BinaryInfo]) {
        match self.sort_order {
            SortOrder::Name => binaries.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOrder::Oldest => binaries.sort_by(|a, b| a.modified_time.cmp(&b.modified_time)),
            SortOrder::Newest => binaries.sort_by(|a, b| b.modified_time.cmp(&a.modified_time)),
        }
    }

    fn print_binaries(&self, binaries: &[BinaryInfo]) {
        if binaries.is_empty() {
            println!("No binaries installed");
        } else {
            let now = SystemTime::now();
            for binary in binaries {
                let time_ago = format_time_ago(now, binary.modified_time);
                println!("{} ({})", binary.name, time_ago);
            }
        }
    }

    fn destination_dir(&self) -> Result<PathBuf> {
        if let Some(ref test_dir) = self.test_dir {
            return Ok(test_dir.clone());
        }

        let home = std::env::var("HOME").map_err(|_| InstallError::HomeNotFound)?;
        Ok(PathBuf::from(home)
            .join(".local")
            .join("softwarewrighter")
            .join("bin"))
    }
}

/// Format time difference as human-readable "time ago" string
fn format_time_ago(now: SystemTime, then: SystemTime) -> String {
    let duration = now
        .duration_since(then)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));

    let seconds = duration.as_secs();

    if seconds < 60 {
        return format!("{} seconds ago", seconds);
    }

    let minutes = seconds / 60;
    if minutes < 60 {
        return format!(
            "{} minute{} ago",
            minutes,
            if minutes == 1 { "" } else { "s" }
        );
    }

    let hours = minutes / 60;
    if hours < 24 {
        return format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" });
    }

    let days = hours / 24;
    if days < 7 {
        return format!("{} day{} ago", days, if days == 1 { "" } else { "s" });
    }

    let weeks = days / 7;
    if weeks < 4 {
        return format!("{} week{} ago", weeks, if weeks == 1 { "" } else { "s" });
    }

    let months = days / 30;
    if months < 12 {
        return format!("{} month{} ago", months, if months == 1 { "" } else { "s" });
    }

    let years = days / 365;
    format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::NormalOutput;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_list_no_binaries() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create empty directory
        fs::create_dir_all(&test_bin_dir).unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 0);
    }

    #[test]
    #[serial]
    fn test_list_single_binary() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create directory with one binary
        fs::create_dir_all(&test_bin_dir).unwrap();
        fs::write(test_bin_dir.join("testapp"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 1);
        assert_eq!(binaries[0], "testapp");
    }

    #[test]
    #[serial]
    fn test_list_multiple_binaries() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create directory with multiple binaries
        fs::create_dir_all(&test_bin_dir).unwrap();
        fs::write(test_bin_dir.join("app1"), "fake binary").unwrap();
        fs::write(test_bin_dir.join("app2"), "fake binary").unwrap();
        fs::write(test_bin_dir.join("app3"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 3);
        // Should be sorted alphabetically
        assert_eq!(binaries[0], "app1");
        assert_eq!(binaries[1], "app2");
        assert_eq!(binaries[2], "app3");
    }

    #[test]
    #[serial]
    fn test_list_ignores_directories() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create directory with binaries and a subdirectory
        fs::create_dir_all(&test_bin_dir).unwrap();
        fs::write(test_bin_dir.join("app1"), "fake binary").unwrap();
        fs::create_dir_all(test_bin_dir.join("subdir")).unwrap();
        fs::write(test_bin_dir.join("app2"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        // Should only include files, not directories
        assert_eq!(binaries.len(), 2);
        assert!(binaries.contains(&"app1".to_string()));
        assert!(binaries.contains(&"app2".to_string()));
    }

    #[test]
    #[serial]
    fn test_list_fails_when_dir_does_not_exist() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("nonexistent");

        // Don't create the directory

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InstallError::InstallDirNotFound(_)
        ));
    }

    #[test]
    #[serial]
    fn test_destination_dir_with_test_dir() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let dest = lister.destination_dir().unwrap();
        assert_eq!(dest, test_bin_dir);
    }

    #[test]
    #[serial]
    fn test_list_sorted_output() {
        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");

        // Create binaries in non-alphabetical order
        fs::create_dir_all(&test_bin_dir).unwrap();
        fs::write(test_bin_dir.join("zebra"), "fake binary").unwrap();
        fs::write(test_bin_dir.join("alpha"), "fake binary").unwrap();
        fs::write(test_bin_dir.join("middle"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Name, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 3);
        // Verify alphabetical sorting
        assert_eq!(binaries[0], "alpha");
        assert_eq!(binaries[1], "middle");
        assert_eq!(binaries[2], "zebra");
    }

    #[test]
    #[serial]
    fn test_sort_by_oldest() {
        use std::thread;
        use std::time::Duration;

        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");
        fs::create_dir_all(&test_bin_dir).unwrap();

        // Create files with different timestamps
        fs::write(test_bin_dir.join("first"), "fake binary").unwrap();
        thread::sleep(Duration::from_millis(100));
        fs::write(test_bin_dir.join("second"), "fake binary").unwrap();
        thread::sleep(Duration::from_millis(100));
        fs::write(test_bin_dir.join("third"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Oldest, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 3);
        // Should be sorted by modification time, oldest first
        assert_eq!(binaries[0], "first");
        assert_eq!(binaries[1], "second");
        assert_eq!(binaries[2], "third");
    }

    #[test]
    #[serial]
    fn test_sort_by_newest() {
        use std::thread;
        use std::time::Duration;

        let temp_home = TempDir::new().unwrap();
        let test_bin_dir = temp_home.path().join("bin");
        fs::create_dir_all(&test_bin_dir).unwrap();

        // Create files with different timestamps
        fs::write(test_bin_dir.join("first"), "fake binary").unwrap();
        thread::sleep(Duration::from_millis(100));
        fs::write(test_bin_dir.join("second"), "fake binary").unwrap();
        thread::sleep(Duration::from_millis(100));
        fs::write(test_bin_dir.join("third"), "fake binary").unwrap();

        let output = NormalOutput;
        let lister = Lister::new(Some(test_bin_dir.clone()), SortOrder::Newest, &output);

        let result = lister.list();
        assert!(result.is_ok());
        let binaries = result.unwrap();
        assert_eq!(binaries.len(), 3);
        // Should be sorted by modification time, newest first
        assert_eq!(binaries[0], "third");
        assert_eq!(binaries[1], "second");
        assert_eq!(binaries[2], "first");
    }

    #[test]
    fn test_format_time_ago_seconds() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(30);
        assert_eq!(format_time_ago(now, then), "30 seconds ago");
    }

    #[test]
    fn test_format_time_ago_minutes() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(120);
        assert_eq!(format_time_ago(now, then), "2 minutes ago");
    }

    #[test]
    fn test_format_time_ago_one_minute() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(60);
        assert_eq!(format_time_ago(now, then), "1 minute ago");
    }

    #[test]
    fn test_format_time_ago_hours() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(3 * 3600);
        assert_eq!(format_time_ago(now, then), "3 hours ago");
    }

    #[test]
    fn test_format_time_ago_days() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(2 * 24 * 3600);
        assert_eq!(format_time_ago(now, then), "2 days ago");
    }

    #[test]
    fn test_format_time_ago_weeks() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(2 * 7 * 24 * 3600);
        assert_eq!(format_time_ago(now, then), "2 weeks ago");
    }

    #[test]
    fn test_format_time_ago_months() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(3 * 30 * 24 * 3600);
        assert_eq!(format_time_ago(now, then), "3 months ago");
    }

    #[test]
    fn test_format_time_ago_years() {
        let now = SystemTime::now();
        let then = now - std::time::Duration::from_secs(2 * 365 * 24 * 3600);
        assert_eq!(format_time_ago(now, then), "2 years ago");
    }

    #[test]
    fn test_sort_order_from_str() {
        assert_eq!("name".parse::<SortOrder>().unwrap(), SortOrder::Name);
        assert_eq!("Name".parse::<SortOrder>().unwrap(), SortOrder::Name);
        assert_eq!("NAME".parse::<SortOrder>().unwrap(), SortOrder::Name);
        assert_eq!("oldest".parse::<SortOrder>().unwrap(), SortOrder::Oldest);
        assert_eq!("newest".parse::<SortOrder>().unwrap(), SortOrder::Newest);
        assert!("invalid".parse::<SortOrder>().is_err());
    }
}
