# Design Document

## sw-install

### Module Structure

```
sw-install/
├── src/
│   ├── main.rs           # Entry point, CLI parsing
│   ├── config.rs         # Configuration and path logic
│   ├── validator.rs      # Project and binary validation
│   ├── installer.rs      # Installation operations
│   ├── output.rs         # Output handling (verbose, dry-run)
│   ├── error.rs          # Error types
│   └── lib.rs            # Library exports for testing
├── tests/
│   └── integration.rs    # Integration tests
└── docs/                 # Documentation
```

### API Design

#### Configuration Module (config.rs)

```rust
pub struct InstallConfig {
    project_path: PathBuf,
    binary_name: Option<String>,
    use_debug: bool,
    verbose: bool,
    dry_run: bool,
}

impl InstallConfig {
    pub fn new(
        project_path: PathBuf,
        binary_name: Option<String>,
        use_debug: bool,
        verbose: bool,
        dry_run: bool,
    ) -> Self;

    pub fn destination_dir(&self) -> Result<PathBuf>;
    pub fn source_binary_path(&self, actual_name: &str) -> PathBuf;
    pub fn destination_binary_path(&self, actual_name: &str) -> Result<PathBuf>;
    pub fn target_subdir(&self) -> &str;
}
```

#### Validator Module (validator.rs)

```rust
pub struct Validator<'a> {
    config: &'a InstallConfig,
    output: Box<dyn OutputHandler>,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: Box<dyn OutputHandler>) -> Self;

    pub fn validate(&self) -> Result<ValidationResult>;

    fn validate_project_path(&self) -> Result<()>;
    fn validate_cargo_toml(&self) -> Result<()>;
    fn extract_binary_name(&self) -> Result<String>;
    fn validate_source_binary(&self, binary_name: &str) -> Result<()>;
}

pub struct ValidationResult {
    pub binary_name: String,
}
```

#### Installer Module (installer.rs)

```rust
pub struct Installer<'a> {
    config: &'a InstallConfig,
    binary_name: String,
    output: Box<dyn OutputHandler>,
}

impl<'a> Installer<'a> {
    pub fn new(
        config: &'a InstallConfig,
        binary_name: String,
        output: Box<dyn OutputHandler>,
    ) -> Self;

    pub fn install(&self) -> Result<()>;

    fn create_destination_dir(&self) -> Result<PathBuf>;
    fn copy_binary(&self, dest_dir: &Path) -> Result<PathBuf>;
    fn set_permissions(&self, binary_path: &Path) -> Result<()>;
}
```

#### Uninstaller Module (uninstaller.rs)

```rust
pub struct Uninstaller {
    binary_name: String,
    verbose: bool,
    dry_run: bool,
    output: Box<dyn OutputHandler>,
}

impl Uninstaller {
    pub fn new(
        binary_name: String,
        verbose: bool,
        dry_run: bool,
        output: Box<dyn OutputHandler>,
    ) -> Self;

    pub fn uninstall(&self) -> Result<()>;

    fn destination_dir(&self) -> Result<PathBuf>;
    fn binary_path(&self) -> Result<PathBuf>;
    fn validate_binary_exists(&self) -> Result<()>;
    fn remove_binary(&self, binary_path: &Path) -> Result<()>;
}
```

#### Output Module (output.rs)

```rust
pub trait OutputHandler: Send + Sync {
    fn info(&self, message: &str);
    fn step(&self, message: &str);
    fn success(&self, message: &str);
    fn error(&self, message: &str);
}

pub struct NormalOutput;
pub struct VerboseOutput;
pub struct DryRunOutput;

impl OutputHandler for NormalOutput { /* ... */ }
impl OutputHandler for VerboseOutput { /* ... */ }
impl OutputHandler for DryRunOutput { /* ... */ }

pub fn create_output_handler(verbose: bool, dry_run: bool) -> Box<dyn OutputHandler>;
```

#### Error Module (error.rs)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Project path does not exist: {0}")]
    ProjectNotFound(PathBuf),

    #[error("Cargo.toml not found in project: {0}")]
    CargoTomlNotFound(PathBuf),

    #[error("Could not parse Cargo.toml: {0}")]
    CargoTomlParse(String),

    #[error("Binary name not found in Cargo.toml")]
    BinaryNameNotFound,

    #[error("Source binary not found: {0}")]
    BinaryNotFound(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid binary name: {0}")]
    InvalidBinaryName(String),

    #[error("Home directory not found")]
    HomeNotFound,
}

pub type Result<T> = std::result::Result<T, InstallError>;
```

### CLI Interface Design

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sw-install")]
#[command(about = "Install softwarewrighter binaries to local PATH", long_about = EXTENDED_HELP)]
#[command(version)]
#[command(author = "Copyright (c) 2025 Michael A Wright")]
struct Args {
    /// Path to the Cargo project (for installation)
    #[arg(short, long, value_name = "PATH", conflicts_with = "uninstall")]
    project: Option<PathBuf>,

    /// Rename the binary during installation
    #[arg(short, long, value_name = "NAME", requires = "project")]
    rename: Option<String>,

    /// Use debug build instead of release
    #[arg(short, long, requires = "project")]
    debug: bool,

    /// Uninstall the named binary
    #[arg(short, long, value_name = "NAME", conflicts_with = "project")]
    uninstall: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Print actions without executing them
    #[arg(short = 'n', long)]
    dry_run: bool,
}

const EXTENDED_HELP: &str = "\
sw-install: Binary Installer for softwarewrighter CLI Projects

OVERVIEW:
  This tool installs compiled Rust binaries from local Cargo projects into
  ~/.local/softwarewrighter/bin/, making them accessible from your PATH.

USAGE MODES:

  1. Install a binary:
     sw-install -p <project-path> [OPTIONS]

  2. Uninstall a binary:
     sw-install -u <binary-name> [OPTIONS]

EXAMPLES:

  Install a release binary:
    sw-install -p ~/projects/ask

  Install with a different name:
    sw-install -p ~/projects/ask -r ask-dev

  Install debug build:
    sw-install -p ~/projects/ask -d

  Preview installation (dry-run):
    sw-install -p ~/projects/ask -n -v

  Uninstall a binary:
    sw-install -u ask

  Uninstall with preview:
    sw-install -u ask -n -v

PREREQUISITES:
  - Project must have a Cargo.toml file
  - Binary must be compiled (run 'cargo build --release' or 'cargo build')
  - ~/.local/softwarewrighter/bin should be in your PATH

WORKFLOW:
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
```

### Execution Flow

```
main()
  ├─> Parse CLI arguments (clap)
  ├─> Create InstallConfig from args
  ├─> Create OutputHandler (normal/verbose/dry-run)
  ├─> Create Validator
  ├─> validator.validate()
  │     ├─> Check project path exists
  │     ├─> Check Cargo.toml exists
  │     ├─> Parse Cargo.toml for binary name
  │     └─> Check binary exists in target/
  ├─> Create Installer with validated binary name
  └─> installer.install()
        ├─> Create destination directory
        ├─> Copy binary
        ├─> Set executable permissions
        └─> Report success
```

### Test Design

#### Unit Tests

**config.rs tests:**
- Test destination_dir() returns ~/.local/softwarewrighter/bin
- Test source_binary_path() with debug/release variants
- Test destination_binary_path() with and without rename
- Test target_subdir() returns "debug" or "release"

**validator.rs tests:**
- Test validation fails for non-existent project path
- Test validation fails for missing Cargo.toml
- Test validation fails for missing binary
- Test validation succeeds with valid project
- Test binary name extraction from Cargo.toml

**installer.rs tests:**
- Test create_destination_dir() creates nested directories
- Test copy_binary() copies file correctly
- Test set_permissions() makes binary executable
- Test dry-run mode doesn't modify filesystem

**output.rs tests:**
- Test NormalOutput only shows success/error
- Test VerboseOutput shows all steps
- Test DryRunOutput prefixes with "Would: "

#### Integration Tests

**tests/integration.rs:**
- Create temporary Cargo project
- Build binary
- Run sw-install on it
- Verify binary installed to correct location
- Verify binary is executable
- Test with --rename option
- Test with --type debug option
- Test with --dry-run option

### Path Resolution Examples

**Example 1: Basic installation**
```
Input:
  --project ~/projects/ask

Computed paths:
  project_path: /Users/mike/projects/ask
  cargo_toml: /Users/mike/projects/ask/Cargo.toml
  source_binary: /Users/mike/projects/ask/target/release/ask
  dest_dir: /Users/mike/.local/softwarewrighter/bin
  dest_binary: /Users/mike/.local/softwarewrighter/bin/ask
```

**Example 2: Debug build with rename**
```
Input:
  --project ~/projects/ask
  --type debug
  --rename ask-dev

Computed paths:
  project_path: /Users/mike/projects/ask
  cargo_toml: /Users/mike/projects/ask/Cargo.toml
  source_binary: /Users/mike/projects/ask/target/debug/ask
  dest_dir: /Users/mike/.local/softwarewrighter/bin
  dest_binary: /Users/mike/.local/softwarewrighter/bin/ask-dev
```

### Error Handling Examples

**Missing project:**
```
Error: Project path does not exist: /Users/mike/projects/nonexistent
```

**Missing Cargo.toml:**
```
Error: Cargo.toml not found in project: /Users/mike/projects/ask
```

**Binary not compiled:**
```
Error: Source binary not found: /Users/mike/projects/ask/target/release/ask
Hint: Run 'cargo build --release' in the project directory
```

### Dry-Run Output Example

```
$ sw-install -p ../ask -n
Would: Check project path exists: /Users/mike/projects/ask
Would: Check Cargo.toml exists: /Users/mike/projects/ask/Cargo.toml
Would: Parse Cargo.toml to extract binary name
Would: Check source binary exists: /Users/mike/projects/ask/target/release/ask
Would: Create destination directory: /Users/mike/.local/softwarewrighter/bin
Would: Copy binary from /Users/mike/projects/ask/target/release/ask
Would: Copy binary to /Users/mike/.local/softwarewrighter/bin/ask
Would: Set executable permissions on /Users/mike/.local/softwarewrighter/bin/ask
Dry-run complete: No changes made
```

### Verbose Output Example

```
$ sw-install -p ../ask -v
[1/7] Checking project path: /Users/mike/projects/ask ... OK
[2/7] Checking Cargo.toml: /Users/mike/projects/ask/Cargo.toml ... OK
[3/7] Parsing Cargo.toml ... binary name: ask
[4/7] Checking source binary: /Users/mike/projects/ask/target/release/ask ... OK
[5/7] Creating destination directory: /Users/mike/.local/softwarewrighter/bin ... OK
[6/7] Copying binary ... OK
[7/7] Setting executable permissions ... OK

Successfully installed: ask
Location: /Users/mike/.local/softwarewrighter/bin/ask
```
