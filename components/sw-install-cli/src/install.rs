// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::path::PathBuf;
use std::process;
use sw_install_core::{InstallConfig, InstallError, NormalOutput};
use sw_install_installer::Installer;
use sw_install_validation::Validator;

pub fn run(
    project_path: PathBuf,
    rename: Option<String>,
    build_type: &str,
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let use_debug = parse_build_type(build_type);
    let output = NormalOutput::new(verbose, dry_run);
    let config = InstallConfig::new(project_path, rename, use_debug, verbose, dry_run, test_dir);
    let result = Validator::new(&config, &output).validate()?;
    Installer::new(
        &config,
        result.binary_name,
        result.source_binary_path,
        &output,
    )
    .install()?;
    Ok(())
}

fn parse_build_type(build_type: &str) -> bool {
    match build_type.to_lowercase().as_str() {
        "debug" => true,
        "release" => false,
        _ => {
            eprintln!(
                "Error: Invalid build type '{}'. Must be 'release' or 'debug'",
                build_type
            );
            process::exit(1);
        }
    }
}
