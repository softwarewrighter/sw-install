// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Output handling for different verbosity and dry-run modes.

/// Trait for handling output in different modes (normal, verbose, dry-run)
pub trait OutputHandler: Send + Sync {
    fn info(&self, message: &str);
    fn step(&self, message: &str);
    fn success(&self, message: &str);
    fn error(&self, message: &str);
}

/// Output mode configuration
#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    /// Normal mode - minimal output
    Normal,
    /// Verbose mode - shows all steps
    Verbose,
    /// Dry-run mode - prefixes messages with "Would: "
    DryRun { verbose: bool },
}

/// Unified output handler supporting all modes
pub struct NormalOutput {
    mode: OutputMode,
}

impl Default for NormalOutput {
    fn default() -> Self {
        Self {
            mode: OutputMode::Normal,
        }
    }
}

impl OutputHandler for NormalOutput {
    fn info(&self, message: &str) {
        match self.mode {
            OutputMode::Normal => {}
            OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { verbose } if verbose => println!("Would: {}", message),
            OutputMode::DryRun { .. } => {}
        }
    }

    fn step(&self, message: &str) {
        match self.mode {
            OutputMode::Normal => {}
            OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { verbose } if verbose => println!("Would: {}", message),
            OutputMode::DryRun { .. } => {}
        }
    }

    fn success(&self, message: &str) {
        match self.mode {
            OutputMode::Normal | OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { .. } => println!("Would: {}", message),
        }
    }

    fn error(&self, message: &str) {
        match self.mode {
            OutputMode::Normal | OutputMode::Verbose => eprintln!("Error: {}", message),
            OutputMode::DryRun { .. } => eprintln!("Would encounter error: {}", message),
        }
    }
}

/// Factory function to create the appropriate output handler
pub fn create_output_handler(verbose: bool, dry_run: bool) -> Box<dyn OutputHandler> {
    let mode = if dry_run {
        OutputMode::DryRun { verbose }
    } else if verbose {
        OutputMode::Verbose
    } else {
        OutputMode::Normal
    };
    Box::new(NormalOutput { mode })
}
