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
pub fn format_time_ago(now: SystemTime, then: SystemTime) -> String {
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
