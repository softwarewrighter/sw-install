# Design Document

## sw-install

### Multi-Component Structure

The project is organized as 7 independent components, each with its own Cargo.toml:

```
sw-install/
├── components/
│   ├── sw-install-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs        # InstallError, Result, re-exports
│   │       ├── config.rs     # InstallConfig struct
│   │       ├── output.rs     # NormalOutput struct
│   │       └── format.rs     # format_time_ago utility
│   │
│   ├── sw-install-workspace/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs        # Workspace binary discovery
│   │
│   ├── sw-install-validation/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs        # Validator, ValidationResult
│   │       ├── detect.rs     # Project type detection
│   │       ├── extract.rs    # Binary name extraction
│   │       └── source.rs     # Source binary validation
│   │
│   ├── sw-install-installer/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs        # Re-exports
│   │       ├── install.rs    # Installer struct
│   │       ├── uninstall.rs  # Uninstaller struct
│   │       └── paths.rs      # Path utilities
│   │
│   ├── sw-install-manage/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs        # Re-exports
│   │       ├── setup.rs      # Setup struct
│   │       └── shell.rs      # Shell configuration
│   │
│   ├── sw-install-list/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs        # Re-exports
│   │       ├── list.rs       # Lister struct
│   │       ├── binaries.rs   # Binary collection
│   │       └── sort.rs       # SortOrder enum
│   │
│   └── sw-install-cli/
│       ├── Cargo.toml
│       ├── build.rs          # Build-time metadata
│       └── src/
│           ├── main.rs       # Entry point, CLI parsing
│           ├── install.rs    # Install command
│           ├── manage.rs     # Setup, list, uninstall commands
│           ├── version.rs    # Version display
│           └── help.txt      # Extended help text
│
├── scripts/
│   └── build.sh              # Build all components
│
└── docs/                     # Documentation
```

### API Design

#### Core Module (sw-install-core)

```rust
// config.rs
pub struct InstallConfig {
    pub project_path: PathBuf,
    pub binary_name: Option<String>,
    pub use_debug: bool,
    pub verbose: bool,
    pub dry_run: bool,
}

impl InstallConfig {
    pub fn new(project_path: PathBuf, binary_name: Option<String>,
               use_debug: bool, verbose: bool, dry_run: bool) -> Self;
    pub fn destination_dir(&self) -> Result<PathBuf>;
    pub fn source_binary_path(&self, actual_name: &str) -> PathBuf;
}

// output.rs
pub struct NormalOutput {
    verbose: bool,
    dry_run: bool,
}

impl NormalOutput {
    pub fn new(verbose: bool, dry_run: bool) -> Self;
    pub fn info(&self, message: &str);
    pub fn success(&self, message: &str);
}

// lib.rs
#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Project path does not exist: {0}")]
    ProjectNotFound(PathBuf),
    // ... other variants
}

pub type Result<T> = std::result::Result<T, InstallError>;

// format.rs
pub fn format_time_ago(now: SystemTime, then: SystemTime) -> String;
```

#### Validation Module (sw-install-validation)

```rust
// lib.rs
pub struct ValidationResult {
    pub binary_name: String,
    pub source_binary_path: PathBuf,
}

pub struct Validator<'a> {
    config: &'a InstallConfig,
    output: &'a NormalOutput,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: &'a NormalOutput) -> Self;
    pub fn validate(&self) -> Result<ValidationResult>;
}

// detect.rs (internal)
pub(crate) enum ProjectType {
    Simple,
    Workspace,
    MultiComponent { component_path: PathBuf },
}

pub(crate) fn detect_project_type(validator: &Validator) -> Result<ProjectType>;
```

#### Installer Module (sw-install-installer)

```rust
// install.rs
pub struct Installer<'a> {
    config: &'a InstallConfig,
    validation: ValidationResult,
    output: &'a NormalOutput,
}

impl<'a> Installer<'a> {
    pub fn new(config: &'a InstallConfig, validation: ValidationResult,
               output: &'a NormalOutput) -> Self;
    pub fn install(&self) -> Result<()>;
}

// uninstall.rs
pub struct Uninstaller<'a> {
    binary_name: String,
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a NormalOutput,
}

impl<'a> Uninstaller<'a> {
    pub fn new(name: String, dry_run: bool, test_dir: Option<PathBuf>,
               out: &'a NormalOutput) -> Self;
    pub fn uninstall(&self) -> Result<()>;
}
```

#### List Module (sw-install-list)

```rust
// sort.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder { Name, Oldest, Newest }

// list.rs
pub struct Lister<'a> {
    test_dir: Option<PathBuf>,
    sort_order: SortOrder,
    output: &'a NormalOutput,
}

impl<'a> Lister<'a> {
    pub fn new(test_dir: Option<PathBuf>, sort_order: SortOrder,
               output: &'a NormalOutput) -> Self;
    pub fn list(&self) -> Result<Vec<String>>;
}
```

#### Manage Module (sw-install-manage)

```rust
// setup.rs
pub struct Setup<'a> {
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: &'a NormalOutput,
}

impl<'a> Setup<'a> {
    pub fn new(dry_run: bool, test_dir: Option<PathBuf>,
               output: &'a NormalOutput) -> Self;
    pub fn setup(&self) -> Result<()>;
}
```

### CLI Interface Design

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sw-install")]
#[command(about = "Install softwarewrighter binaries to local PATH")]
struct Args {
    #[arg(short, long, value_name = "PATH")]
    project: Option<PathBuf>,

    #[arg(short, long, value_name = "NAME")]
    rename: Option<String>,

    #[arg(long, default_value = "release")]
    r#type: String,

    #[arg(short, long, value_name = "NAME")]
    uninstall: Option<String>,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long, default_value = "name")]
    sort: String,

    #[arg(long)]
    setup_install_dir: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short = 'n', long)]
    dry_run: bool,

    #[arg(short, long)]
    test_dir: Option<PathBuf>,
}
```

### Execution Flow

```
main()
  ├─> Parse CLI arguments (clap)
  ├─> Dispatch based on operation:
  │
  ├─> Install operation (-p):
  │     ├─> Create InstallConfig
  │     ├─> Create NormalOutput
  │     ├─> Create Validator
  │     ├─> validator.validate()
  │     │     ├─> Detect project type
  │     │     ├─> Extract binary name
  │     │     └─> Validate source binary
  │     ├─> Create Installer
  │     └─> installer.install()
  │           ├─> Create destination directory
  │           ├─> Copy binary
  │           └─> Set permissions
  │
  ├─> List operation (-l):
  │     ├─> Create Lister
  │     └─> lister.list()
  │
  ├─> Uninstall operation (-u):
  │     ├─> Create Uninstaller
  │     └─> uninstaller.uninstall()
  │
  └─> Setup operation (--setup-install-dir):
        ├─> Create Setup
        └─> setup.setup()
```

### Build System

#### scripts/build.sh

Builds all components in dependency order:

```bash
#!/bin/bash
set -e

# Build order: core -> workspace -> validation -> installer -> manage -> list -> cli
cd components/sw-install-core && cargo build --release
cd ../sw-install-workspace && cargo build --release
cd ../sw-install-validation && cargo build --release
cd ../sw-install-installer && cargo build --release
cd ../sw-install-manage && cargo build --release
cd ../sw-install-list && cargo build --release
cd ../sw-install-cli && cargo build --release

echo "Binary: components/sw-install-cli/target/release/sw-install"
```

#### build.rs (sw-install-cli)

Captures build-time metadata:

```rust
fn main() {
    // Capture hostname, git commit, timestamp
    println!("cargo:rustc-env=BUILD_HOST={}", hostname);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}
```

### sw-standards Compliance

The codebase is organized to meet sw-checklist requirements:

| Metric | Threshold | Status |
|--------|-----------|--------|
| Functions per module | ≤4 (warn), ≤7 (fail) | Pass |
| Modules per crate | ≤4 (warn), ≤7 (fail) | Pass |
| Lines per function | ≤25 (warn), ≤50 (fail) | Pass |
| Lines per file | ≤350 (warn), ≤500 (fail) | Pass |

**Current: 45 checks passed, 0 failed, 0 code quality warnings**

### Test Strategy

Tests are located in each component's src/ directory and in the CLI's tests/ directory.

- Unit tests: Test individual functions in isolation
- Integration tests: Test full workflows with temp directories
- Test isolation: Use serial_test for filesystem operations
