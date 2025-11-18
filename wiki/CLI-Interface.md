# CLI Interface

The CLI Interface module (`main.rs`) serves as the entry point for the sw-install application.

## Responsibilities

- Parse command-line arguments using clap
- Display version information
- Route to appropriate operations (install, uninstall, list, setup)
- Handle top-level error display and exit codes

## Implementation

### Argument Structure

```rust
#[derive(Parser, Debug)]
#[command(name = "sw-install")]
#[command(about = "Install softwarewrighter binaries to local PATH")]
struct Args {
    /// Path to the Cargo project (for installation)
    #[arg(short, long, value_name = "PATH")]
    project: Option<PathBuf>,

    /// Rename the binary during installation
    #[arg(short, long, value_name = "NAME")]
    rename: Option<String>,

    /// Build type to install (release or debug)
    #[arg(long, value_name = "TYPE", default_value = "release")]
    r#type: String,

    /// Uninstall the named binary
    #[arg(short, long, value_name = "NAME")]
    uninstall: Option<String>,

    /// List all installed binaries
    #[arg(short, long)]
    list: bool,

    /// Sort order for list: name, oldest, newest
    #[arg(short, long, value_name = "ORDER", default_value = "name")]
    sort: String,

    /// Setup installation directory and configure PATH
    #[arg(long)]
    setup_install_dir: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Print actions without executing them
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Override destination directory for testing
    #[arg(short = 't', long, value_name = "DIR")]
    test_dir: Option<PathBuf>,
}
```

## Operation Routing

The main function determines which operation to perform based on the arguments:

```
┌─────────────────────────────────────────┐
│          main() Entry Point             │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│         Parse Arguments (clap)          │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│          Operation Dispatch             │
│                                         │
│  if --setup-install-dir → Setup         │
│  else if --list → List                  │
│  else if --uninstall → Uninstall        │
│  else if --project → Install            │
│  else → Error (no operation specified)  │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│         Execute Operation               │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│         Handle Result                   │
│   Ok(_) → exit(0)                       │
│   Err(e) → print error, exit(1)         │
└─────────────────────────────────────────┘
```

## Version Information

The CLI provides enhanced version information including build metadata:

```rust
fn print_version() {
    println!(
        "{} {}
{}
License: {}
Repository: {}

Build Information:
  Host: {}
  Commit: {}
  Timestamp: {}",
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
```

### Example Output

```
$ sw-install --version
sw-install 0.1.0
Copyright (c) 2025 Michael A Wright
License: MIT
Repository: https://github.com/softwarewrighter/sw-install

Build Information:
  Host: manager
  Commit: c661d31
  Timestamp: 2025-01-17T09:15:38-0800
```

## Error Handling

The main function uses a wrapper function to handle errors:

```rust
fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
```

This ensures:
- Clean error messages for users
- Proper exit codes for scripting
- All errors are caught and displayed

## Usage Examples

### Install a Binary

```bash
sw-install -p ~/projects/my-tool
```

### Install with Rename

```bash
sw-install -p ~/projects/ask --rename ask-dev
```

### Install Debug Build

```bash
sw-install -p ~/projects/my-tool --type debug
```

### Uninstall a Binary

```bash
sw-install -u my-tool
```

### List Installed Binaries

```bash
sw-install --list
sw-install --list --sort newest
```

### Setup Installation Directory

```bash
sw-install --setup-install-dir
```

### Dry-Run Mode

```bash
sw-install -p ~/projects/my-tool --dry-run --verbose
```

## Argument Validation

Clap provides automatic validation:

- **Mutual exclusivity**: `--project` and `--uninstall` cannot be used together
- **Dependencies**: `--rename` requires `--project`
- **Type checking**: Paths and strings are validated
- **Help generation**: Automatic `--help` and `-h` flags

## Extended Help

The application provides comprehensive help text with:

- Overview and usage modes
- Examples for common scenarios
- Prerequisites and workflow explanation
- AI agent guidance
- Error handling information
- Security notes

Access via:
```bash
sw-install --help
```

## Integration with Other Components

```
CLI Interface
    ↓
    ├─→ Config (builds InstallConfig)
    ├─→ Validator (validates preconditions)
    ├─→ Installer (performs installation)
    ├─→ Uninstaller (removes binaries)
    ├─→ Lister (displays binaries)
    ├─→ Setup (configures PATH)
    └─→ OutputHandler (displays results)
```

## Testing

The CLI interface is tested through:

1. **Unit tests**: Argument parsing validation
2. **Integration tests**: End-to-end command execution
3. **Manual testing**: User acceptance testing

### Example Integration Test

```rust
#[test]
fn test_install_command() {
    let output = Command::new("sw-install")
        .args(&["-p", "test-project", "-n", "-v"])
        .output()
        .expect("Failed to execute");

    assert!(output.status.success());
}
```

## See Also

- [Configuration](Configuration) - How arguments are transformed into config
- [Output Handler](Output-Handler) - Output formatting based on flags
- [Error Handling](Error-Handling) - Error types and display
- [Testing Strategy](Testing-Strategy) - Testing approaches
