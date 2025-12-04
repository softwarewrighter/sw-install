// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::path::PathBuf;
use std::process;
use sw_install_core::{InstallError, NormalOutput};
use sw_install_installer::Uninstaller;
use sw_install_list::{Lister, SortOrder};
use sw_install_manage::Setup;

pub fn run_setup(
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let output = NormalOutput::new(verbose, dry_run);
    Setup::new(dry_run, test_dir, &output).setup()
}

pub fn run_list(
    verbose: bool,
    sort_order_str: &str,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let output = NormalOutput::new(verbose, false);
    let sort_order = match sort_order_str.parse::<SortOrder>() {
        Ok(order) => order,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    Lister::new(test_dir, sort_order, &output).list()?;
    Ok(())
}

pub fn run_uninstall(
    binary_name: String,
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let output = NormalOutput::new(verbose, dry_run);
    Uninstaller::new(binary_name, dry_run, test_dir, &output).uninstall()
}
