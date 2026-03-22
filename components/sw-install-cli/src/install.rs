// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::process;
use sw_install_core::{InstallConfig, InstallError, NormalOutput};
use sw_install_installer::Installer;
use sw_install_validation::Validator;

pub fn run(config: InstallConfig) -> Result<(), InstallError> {
    let output = NormalOutput::new(config.verbose, config.dry_run);
    if config.build {
        run_cargo_build(&config, &output)?;
    }
    let result = Validator::new(&config, &output).validate()?;
    validate_rename(&config, result.binaries.len())?;
    for (name, source_path) in &result.binaries {
        Installer::new(&config, name.clone(), source_path.clone(), &output).install()?;
    }
    Ok(())
}

fn validate_rename(config: &InstallConfig, count: usize) -> Result<(), InstallError> {
    if config.rename.is_some() && count > 1 {
        return Err(InstallError::RenameMultipleBinaries(count));
    }
    Ok(())
}

fn run_cargo_build(config: &InstallConfig, output: &NormalOutput) -> Result<(), InstallError> {
    let build_type = if config.use_debug { "debug" } else { "release" };
    output.info(&format!("Running cargo build --{build_type}..."));
    if config.dry_run {
        return Ok(());
    }
    let mut cmd = process::Command::new("cargo");
    cmd.arg("build").current_dir(&config.project_path);
    if !config.use_debug {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err(InstallError::BuildFailed);
    }
    Ok(())
}

pub fn parse_build_type(build_type: &str) -> bool {
    match build_type.to_lowercase().as_str() {
        "debug" => true,
        "release" => false,
        _ => {
            eprintln!("Error: Invalid build type '{build_type}'. Must be 'release' or 'debug'");
            process::exit(1);
        }
    }
}
