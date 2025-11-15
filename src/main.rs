// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use clap::Parser;
use std::path::PathBuf;
use std::process;
use sw_install::{
    create_output_handler, InstallConfig, InstallError, Installer, Setup, Uninstaller, Validator,
};

const REPOSITORY: &str = "https://github.com/softwarewrighter/sw-install";
const LICENSE: &str = "MIT";
const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";

fn print_version() {
    println!(
        "{} {}\n{}\nLicense: {}\nRepository: {}\n\nBuild Information:\n  Host: {}\n  Commit: {}\n  Timestamp: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        COPYRIGHT,
        LICENSE,
        REPOSITORY,
        env!("BUILD_HOST"),
        env!("GIT_HASH"),
        env!("BUILD_TIMESTAMP")
    );
}

const EXTENDED_HELP: &str = "\
sw-install: Binary Installer for softwarewrighter CLI Projects

OVERVIEW:
  This tool installs compiled Rust binaries from local Cargo projects into
  ~/.local/softwarewrighter/bin/, making them accessible from your PATH.

USAGE MODES:

  1. Setup installation directory (first-time use):
     sw-install --setup-install-dir

  2. Install a binary:
     sw-install -p <project-path> [OPTIONS]

  3. Uninstall a binary:
     sw-install -u <binary-name> [OPTIONS]

EXAMPLES:

  First-time setup:
    sw-install --setup-install-dir
    source ~/.bashrc  # or ~/.zshrc

  Install a release binary:
    sw-install -p ~/projects/ask

  Install with a different name:
    sw-install -p ~/projects/ask -r ask-dev

  Install debug build:
    sw-install -p ~/projects/ask --type debug

  Preview installation (dry-run):
    sw-install -p ~/projects/ask -n -v

  Uninstall a binary:
    sw-install -u ask

  Uninstall with preview:
    sw-install -u ask -n -v

PREREQUISITES:
  - Run 'sw-install --setup-install-dir' for first-time setup
  - Project must have a Cargo.toml file
  - Binary must be compiled (run 'cargo build --release' or 'cargo build')

WORKFLOW:

  Setup (first time):
  1. Creates ~/.local/softwarewrighter/bin/ directory
  2. Detects your shell configuration file (~/.zshrc or ~/.bashrc)
  3. Adds PATH configuration to shell config
  4. Provides instructions to reload shell

  Installation:
  1. Validates project path and Cargo.toml
  2. Extracts binary name from Cargo.toml
  3. Verifies compiled binary exists in target/release or target/debug
  4. Creates destination directory if needed
  5. Copies binary to ~/.local/softwarewrighter/bin/
  6. Sets executable permissions

AI AGENT GUIDANCE:
  This tool is designed for automated binary installation in development
  workflows. Key features for automation:
  - Use --dry-run (-n) to preview actions before execution
  - Use --verbose (-v) to see detailed step-by-step output
  - Check exit codes: 0 = success, non-zero = error
  - Combine flags: -nvp for verbose dry-run installation
  - All file paths are validated before operations
  - Errors include actionable suggestions

ERROR HANDLING:
  - Missing project: 'Project path does not exist'
  - Missing Cargo.toml: 'Cargo.toml not found in project'
  - Binary not built: 'Source binary not found' (suggests running cargo build)
  - Permission errors: Reports specific file/directory issues

SECURITY:
  - Operates only in user-owned directories
  - No privilege escalation required
  - Validates all paths to prevent traversal attacks
  - Safe to run in automated environments
";

#[derive(Parser, Debug)]
#[command(name = "sw-install")]
#[command(about = "Install softwarewrighter binaries to local PATH", long_about = EXTENDED_HELP)]
#[command(disable_version_flag = true)]
struct Args {
    /// Path to the Cargo project (for installation)
    #[arg(short, long, value_name = "PATH", conflicts_with = "uninstall")]
    project: Option<PathBuf>,

    /// Rename the binary during installation
    #[arg(short, long, value_name = "NAME", requires = "project")]
    rename: Option<String>,

    /// Build type to install (release or debug)
    #[arg(
        long,
        value_name = "TYPE",
        default_value = "release",
        requires = "project"
    )]
    r#type: String,

    /// Uninstall the named binary
    #[arg(short, long, value_name = "NAME", conflicts_with = "project")]
    uninstall: Option<String>,

    /// Setup installation directory and configure PATH
    #[arg(short = 's', long, conflicts_with_all = ["project", "uninstall"])]
    setup_install_dir: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Print actions without executing them
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Override destination directory for testing purposes
    #[arg(short = 't', long, value_name = "DIR")]
    test_dir: Option<PathBuf>,

    /// Print version information
    #[arg(short = 'V', long)]
    version: bool,
}

fn main() {
    let args = Args::parse();

    if args.version {
        print_version();
        return;
    }

    let result = if args.setup_install_dir {
        run_setup(args.verbose, args.dry_run, args.test_dir)
    } else if let Some(binary_name) = args.uninstall {
        run_uninstall(binary_name, args.verbose, args.dry_run, args.test_dir)
    } else if let Some(project_path) = args.project {
        // Validate build type
        let use_debug = match args.r#type.to_lowercase().as_str() {
            "debug" => true,
            "release" => false,
            _ => {
                eprintln!(
                    "Error: Invalid build type '{}'. Must be 'release' or 'debug'",
                    args.r#type
                );
                process::exit(1);
            }
        };

        run_install(
            project_path,
            args.rename,
            use_debug,
            args.verbose,
            args.dry_run,
            args.test_dir,
        )
    } else {
        Err(InstallError::NoOperationSpecified)
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_setup(verbose: bool, dry_run: bool, test_dir: Option<PathBuf>) -> Result<(), InstallError> {
    let output = create_output_handler(verbose, dry_run);
    let setup = Setup::new(dry_run, test_dir, output.as_ref());
    setup.setup()?;

    Ok(())
}

fn run_install(
    project_path: PathBuf,
    rename: Option<String>,
    use_debug: bool,
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let output = create_output_handler(verbose, dry_run);
    let config = InstallConfig::new(project_path, rename, use_debug, verbose, dry_run, test_dir);

    // Validation phase
    let validator = Validator::new(&config, output.as_ref());
    let validation_result = validator.validate()?;

    // Installation phase
    let installer = Installer::new(&config, validation_result.binary_name, output.as_ref());
    installer.install()?;

    Ok(())
}

fn run_uninstall(
    binary_name: String,
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
) -> Result<(), InstallError> {
    let output = create_output_handler(verbose, dry_run);
    let uninstaller = Uninstaller::new(binary_name, dry_run, test_dir, output.as_ref());
    uninstaller.uninstall()?;

    Ok(())
}
