// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use crate::binaries::{collect_binaries, get_bin_dir};
use crate::sort::SortOrder;
use std::path::PathBuf;
use std::time::SystemTime;
use sw_install_core::format_time_ago;
use sw_install_core::{NormalOutput, Result};

pub struct Lister<'a> {
    test_dir: Option<PathBuf>,
    sort_order: SortOrder,
    output: &'a NormalOutput,
}

impl<'a> Lister<'a> {
    pub fn new(test_dir: Option<PathBuf>, sort_order: SortOrder, output: &'a NormalOutput) -> Self {
        Self {
            test_dir,
            sort_order,
            output,
        }
    }

    pub fn list(&self) -> Result<Vec<String>> {
        self.output.info("Listing installed binaries...");
        let bin_dir = get_bin_dir(&self.test_dir)?;
        let mut bins = collect_binaries(&bin_dir)?;
        sort_binaries(&mut bins, self.sort_order);
        print_binaries(&bins);
        Ok(bins.into_iter().map(|(n, _)| n).collect())
    }
}

fn sort_binaries(bins: &mut [(String, SystemTime)], order: SortOrder) {
    match order {
        SortOrder::Name => bins.sort_by(|a, b| a.0.cmp(&b.0)),
        SortOrder::Oldest => bins.sort_by(|a, b| a.1.cmp(&b.1)),
        SortOrder::Newest => bins.sort_by(|a, b| b.1.cmp(&a.1)),
    }
}

fn print_binaries(bins: &[(String, SystemTime)]) {
    if bins.is_empty() {
        println!("No binaries installed");
        return;
    }
    let now = SystemTime::now();
    for (name, time) in bins {
        println!("{} ({})", name, format_time_ago(now, *time));
    }
}
