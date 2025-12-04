// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

#[derive(Debug, Clone, Copy)]
enum OutputMode {
    Normal,
    Verbose,
    DryRun { verbose: bool },
}

pub struct NormalOutput {
    mode: OutputMode,
}

impl NormalOutput {
    pub fn new(verbose: bool, dry_run: bool) -> Self {
        let mode = match (dry_run, verbose) {
            (true, v) => OutputMode::DryRun { verbose: v },
            (false, true) => OutputMode::Verbose,
            (false, false) => OutputMode::Normal,
        };
        Self { mode }
    }

    pub fn info(&self, message: &str) {
        match self.mode {
            OutputMode::Normal => {}
            OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { verbose: true } => println!("Would: {}", message),
            OutputMode::DryRun { verbose: false } => {}
        }
    }

    pub fn success(&self, message: &str) {
        match self.mode {
            OutputMode::Normal | OutputMode::Verbose => println!("{}", message),
            OutputMode::DryRun { .. } => println!("Would: {}", message),
        }
    }
}

impl Default for NormalOutput {
    fn default() -> Self {
        Self::new(false, false)
    }
}
