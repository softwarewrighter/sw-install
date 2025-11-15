// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

/// Trait for handling output in different modes (normal, verbose, dry-run)
pub trait OutputHandler: Send + Sync {
    fn info(&self, message: &str);
    fn step(&self, message: &str);
    fn success(&self, message: &str);
    fn error(&self, message: &str);
}

/// Normal output mode - minimal output
pub struct NormalOutput;

impl OutputHandler for NormalOutput {
    fn info(&self, _message: &str) {
        // Normal mode doesn't show info messages
    }

    fn step(&self, _message: &str) {
        // Normal mode doesn't show step messages
    }

    fn success(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }
}

/// Verbose output mode - shows all steps
pub struct VerboseOutput;

impl OutputHandler for VerboseOutput {
    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn step(&self, message: &str) {
        println!("{}", message);
    }

    fn success(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }
}

/// Dry-run output mode - prefixes all messages with "Would: "
pub struct DryRunOutput {
    verbose: bool,
}

impl DryRunOutput {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl OutputHandler for DryRunOutput {
    fn info(&self, message: &str) {
        if self.verbose {
            println!("Would: {}", message);
        }
    }

    fn step(&self, message: &str) {
        if self.verbose {
            println!("Would: {}", message);
        }
    }

    fn success(&self, message: &str) {
        println!("Would: {}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("Would encounter error: {}", message);
    }
}

/// Factory function to create the appropriate output handler
pub fn create_output_handler(verbose: bool, dry_run: bool) -> Box<dyn OutputHandler> {
    if dry_run {
        Box::new(DryRunOutput::new(verbose))
    } else if verbose {
        Box::new(VerboseOutput)
    } else {
        Box::new(NormalOutput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_normal_output() {
        let output = create_output_handler(false, false);
        // Test that it was created successfully (we can't easily test output without capturing stdout)
        output.success("test");
    }

    #[test]
    fn test_create_verbose_output() {
        let output = create_output_handler(true, false);
        output.step("test");
    }

    #[test]
    fn test_create_dry_run_output() {
        let output = create_output_handler(false, true);
        output.success("test");
    }

    #[test]
    fn test_create_verbose_dry_run_output() {
        let output = create_output_handler(true, true);
        output.step("test");
    }
}
