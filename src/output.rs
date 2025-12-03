// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Output handling for different verbosity and dry-run modes.

/// Output mode configuration
#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Normal,
    Verbose,
    DryRun { verbose: bool },
}

/// Output handler supporting all modes
pub struct NormalOutput {
    mode: OutputMode,
}

impl NormalOutput {
    pub fn new(verbose: bool, dry_run: bool) -> Self {
        let mode = if dry_run {
            OutputMode::DryRun { verbose }
        } else if verbose {
            OutputMode::Verbose
        } else {
            OutputMode::Normal
        };
        Self { mode }
    }

    pub fn info(&self, message: &str) {
        match self.mode {
            OutputMode::Normal => {}
            OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { verbose } if verbose => println!("Would: {}", message),
            OutputMode::DryRun { .. } => {}
        }
    }

    pub fn step(&self, message: &str) {
        self.info(message);
    }

    pub fn success(&self, message: &str) {
        match self.mode {
            OutputMode::Normal | OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { .. } => println!("Would: {}", message),
        }
    }

    pub fn error(&self, message: &str) {
        match self.mode {
            OutputMode::Normal | OutputMode::Verbose => eprintln!("Error: {}", message),
            OutputMode::DryRun { .. } => eprintln!("Would encounter error: {}", message),
        }
    }
}

impl Default for NormalOutput {
    fn default() -> Self {
        Self::new(false, false)
    }
}
