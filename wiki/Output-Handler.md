# Output Handler

The Output Handler module (`output.rs`) provides an abstraction layer for different output modes using the Strategy pattern.

## Responsibilities

- Define output interface (trait)
- Implement different output strategies (Normal, Verbose, DryRun)
- Provide factory function for creating handlers
- Ensure consistent output formatting across operations

## Architecture

```
┌────────────────────────────────────┐
│     OutputHandler Trait            │
│  (Abstract Interface)              │
└───────────────┬────────────────────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    v           v           v
┌────────┐ ┌─────────┐ ┌─────────┐
│ Normal │ │ Verbose │ │ DryRun  │
│ Output │ │ Output  │ │ Output  │
└────────┘ └─────────┘ └─────────┘
```

## Trait Definition

```rust
pub trait OutputHandler: Send + Sync {
    /// Display informational message
    fn info(&self, message: &str);

    /// Display step in process
    fn step(&self, message: &str);

    /// Display success message
    fn success(&self, message: &str);

    /// Display error message
    fn error(&self, message: &str);
}
```

### Method Purposes

- **info**: General information (paths, settings, etc.)
- **step**: Progress indication (step 1/7, checking..., etc.)
- **success**: Operation completion
- **error**: Error messages (though errors usually use eprintln!)

## Implementations

### Normal Output

Minimal output for regular use:

```rust
pub struct NormalOutput;

impl OutputHandler for NormalOutput {
    fn info(&self, message: &str) {
        // Suppress info in normal mode
    }

    fn step(&self, message: &str) {
        // Suppress steps in normal mode
    }

    fn success(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("{}", message);
    }
}
```

**Characteristics:**
- Shows only success and error messages
- Minimal noise for scripting
- Clean, concise output

**Example:**
```
Successfully installed: my-tool
Location: /home/user/.local/softwarewrighter/bin/my-tool
```

### Verbose Output

Detailed step-by-step output:

```rust
pub struct VerboseOutput;

impl OutputHandler for VerboseOutput {
    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn step(&self, message: &str) {
        println!("{}", message);
    }

    fn success(&self, message: &str) {
        println!("\n{}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("{}", message);
    }
}
```

**Characteristics:**
- Shows all messages
- Step-by-step progress
- Useful for debugging

**Example:**
```
[1/7] Checking project path: /home/user/projects/my-tool ... OK
[2/7] Checking Cargo.toml: /home/user/projects/my-tool/Cargo.toml ... OK
[3/7] Parsing Cargo.toml ... binary name: my-tool
[4/7] Checking source binary: /home/user/projects/my-tool/target/release/my-tool ... OK
[5/7] Creating destination directory: /home/user/.local/softwarewrighter/bin ... OK
[6/7] Copying binary ... OK
[7/7] Setting executable permissions ... OK

Successfully installed: my-tool
Location: /home/user/.local/softwarewrighter/bin/my-tool
```

### Dry-Run Output

Preview mode without actual file operations:

```rust
pub struct DryRunOutput;

impl OutputHandler for DryRunOutput {
    fn info(&self, message: &str) {
        println!("Would: {}", message);
    }

    fn step(&self, message: &str) {
        println!("Would: {}", message);
    }

    fn success(&self, message: &str) {
        println!("Would: {}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("Would fail: {}", message);
    }
}
```

**Characteristics:**
- Prefixes all output with "Would:"
- No actual file operations performed
- Safe preview of actions

**Example:**
```
Would: Check project path exists: /home/user/projects/my-tool
Would: Check Cargo.toml exists: /home/user/projects/my-tool/Cargo.toml
Would: Parse Cargo.toml to extract binary name
Would: Check source binary exists: /home/user/projects/my-tool/target/release/my-tool
Would: Create destination directory: /home/user/.local/softwarewrighter/bin
Would: Copy binary from /home/user/projects/my-tool/target/release/my-tool
Would: Copy binary to /home/user/.local/softwarewrighter/bin/my-tool
Would: Set executable permissions on /home/user/.local/softwarewrighter/bin/my-tool
Dry-run complete: No changes made
```

## Factory Function

```rust
pub fn create_output_handler(verbose: bool, dry_run: bool) -> Box<dyn OutputHandler> {
    if dry_run {
        Box::new(DryRunOutput)
    } else if verbose {
        Box::new(VerboseOutput)
    } else {
        Box::new(NormalOutput)
    }
}
```

**Priority:**
1. Dry-run mode (highest priority)
2. Verbose mode
3. Normal mode (default)

**Note:** If both `--dry-run` and `--verbose` are specified, dry-run takes precedence.

## Usage Examples

### In Main

```rust
fn main() -> Result<()> {
    let args = Args::parse();
    let output = create_output_handler(args.verbose, args.dry_run);

    // Use output throughout application
    output.info("Starting installation...");
    // ...
}
```

### In Validator

```rust
impl<'a> Validator<'a> {
    pub fn validate(&self) -> Result<ValidationResult> {
        self.output.step("Checking project path");
        // ... validation logic

        self.output.info("Binary name: my-tool");
        // ...
    }
}
```

### In Installer

```rust
impl<'a> Installer<'a> {
    pub fn install(&self) -> Result<()> {
        self.output.step("Creating destination directory");
        // ... create directory

        self.output.success("Successfully installed: my-tool");
        // ...
    }
}
```

## Design Pattern: Strategy Pattern

The OutputHandler implements the **Strategy Pattern**:

```
Context           Strategy Interface    Concrete Strategies
────────          ──────────────────    ──────────────────

Installer    →    OutputHandler     ←   NormalOutput
Validator    →    (trait)           ←   VerboseOutput
Uninstaller  →                      ←   DryRunOutput
```

**Benefits:**
1. **Runtime selection**: Choose output mode at runtime
2. **Decoupling**: Business logic independent of output format
3. **Extensibility**: Easy to add new output modes
4. **Testability**: Can inject test output handler

## Testing Output

### Test Output Handler

```rust
#[cfg(test)]
pub struct TestOutput {
    messages: Arc<Mutex<Vec<String>>>,
}

impl TestOutput {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }
}

impl OutputHandler for TestOutput {
    fn info(&self, message: &str) {
        self.messages.lock().unwrap().push(format!("INFO: {}", message));
    }

    fn step(&self, message: &str) {
        self.messages.lock().unwrap().push(format!("STEP: {}", message));
    }

    fn success(&self, message: &str) {
        self.messages.lock().unwrap().push(format!("SUCCESS: {}", message));
    }

    fn error(&self, message: &str) {
        self.messages.lock().unwrap().push(format!("ERROR: {}", message));
    }
}
```

**Usage in Tests:**
```rust
#[test]
fn test_validator_output() {
    let output = Box::new(TestOutput::new());
    let output_ref = output.clone();
    let validator = Validator::new(&config, output);

    validator.validate().unwrap();

    let messages = output_ref.get_messages();
    assert!(messages.contains(&"STEP: Checking project path".to_string()));
}
```

## Integration with Components

All major components use OutputHandler:

1. **Validator**: Reports validation steps
2. **Installer**: Reports installation progress
3. **Uninstaller**: Reports removal progress
4. **Lister**: Reports list output
5. **Setup**: Reports setup progress

## Design Decisions

### Why a Trait Instead of Enum?

**Trait Approach (Current):**
```rust
Box<dyn OutputHandler>
```

**Enum Approach (Alternative):**
```rust
enum OutputMode { Normal, Verbose, DryRun }
```

**Reasons for Trait:**
1. **Open/Closed Principle**: Easy to add new modes without modifying existing code
2. **Polymorphism**: Runtime selection of behavior
3. **Testability**: Can inject custom test handlers
4. **Separation of Concerns**: Each mode is self-contained

### Why Send + Sync?

```rust
pub trait OutputHandler: Send + Sync
```

**Reasons:**
- **Send**: Can be transferred between threads
- **Sync**: Can be shared between threads
- **Future-proofing**: Enables concurrent operations if needed
- **Arc compatibility**: Can wrap in Arc for sharing

### Why Box<dyn OutputHandler>?

**Reasons:**
1. **Heap allocation**: Trait objects require boxing
2. **Dynamic dispatch**: Runtime polymorphism
3. **Size unknown at compile time**: Different implementations have different sizes

## Comparison Table

| Feature       | Normal | Verbose | DryRun |
|---------------|--------|---------|--------|
| info()        | ✗      | ✓       | ✓      |
| step()        | ✗      | ✓       | ✓      |
| success()     | ✓      | ✓       | ✓      |
| error()       | ✓      | ✓       | ✓      |
| File ops      | ✓      | ✓       | ✗      |
| Use case      | Scripts| Debug   | Preview|

## See Also

- [CLI Interface](CLI-Interface) - Creates output handler from args
- [Validator](Validator) - Uses output for validation steps
- [Installer](Installer) - Uses output for installation progress
- [Testing Strategy](Testing-Strategy) - Test output handler usage
