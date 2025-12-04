// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

mod install;
mod manage;
mod version;

use clap::Parser;
use std::path::PathBuf;
use std::process;
use sw_install_core::InstallError;

const EXTENDED_HELP: &str = include_str!("help.txt");

#[derive(Parser, Debug)]
#[command(name = "sw-install")]
#[command(about = "Install softwarewrighter binaries to local PATH", long_about = EXTENDED_HELP)]
#[command(disable_version_flag = true)]
struct Args {
    #[arg(short, long, value_name = "PATH", conflicts_with = "uninstall")]
    project: Option<PathBuf>,
    #[arg(short, long, value_name = "NAME", requires = "project")]
    rename: Option<String>,
    #[arg(
        long,
        value_name = "TYPE",
        default_value = "release",
        requires = "project"
    )]
    r#type: String,
    #[arg(short, long, value_name = "NAME", conflicts_with = "project")]
    uninstall: Option<String>,
    #[arg(short = 'l', long, conflicts_with_all = ["project", "uninstall"])]
    list: bool,
    #[arg(
        short = 's',
        long,
        value_name = "ORDER",
        default_value = "name",
        requires = "list"
    )]
    sort: String,
    #[arg(long, conflicts_with_all = ["project", "uninstall", "list"])]
    setup_install_dir: bool,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short = 'n', long)]
    dry_run: bool,
    #[arg(short = 't', long, value_name = "DIR")]
    test_dir: Option<PathBuf>,
    #[arg(short = 'V', long)]
    version: bool,
}

fn main() {
    let args = Args::parse();
    if args.version {
        version::print();
        return;
    }
    let result = dispatch(&args);
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn dispatch(args: &Args) -> Result<(), InstallError> {
    if args.setup_install_dir {
        manage::run_setup(args.verbose, args.dry_run, args.test_dir.clone())
    } else if args.list {
        manage::run_list(args.verbose, &args.sort, args.test_dir.clone())
    } else if let Some(ref binary_name) = args.uninstall {
        manage::run_uninstall(
            binary_name.clone(),
            args.verbose,
            args.dry_run,
            args.test_dir.clone(),
        )
    } else if let Some(ref project_path) = args.project {
        install::run(
            project_path.clone(),
            args.rename.clone(),
            &args.r#type,
            args.verbose,
            args.dry_run,
            args.test_dir.clone(),
        )
    } else {
        Err(InstallError::NoOperationSpecified)
    }
}
