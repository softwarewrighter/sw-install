# Architecture

## sw-install

### Overview
sw-install is a command-line tool that installs compiled Rust binaries from local Cargo projects into a user-specific installation directory.

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                      │
│              (clap argument parser)                  │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│               Configuration Layer                    │
│  - Parse arguments                                   │
│  - Build InstallConfig                               │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│              Validation Layer                        │
│  - Verify project path exists                        │
│  - Verify Cargo.toml exists                          │
│  - Verify binary exists in target/                   │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│              Installation Layer                      │
│  - Create destination directory                      │
│  - Copy binary                                       │
│  - Set executable permissions                        │
└─────────────────────────────────────────────────────┘
```

### Component Design

#### CLI Interface (main.rs)
- Entry point for the application
- Uses clap crate for argument parsing
- Delegates to InstallConfig for business logic

#### Configuration (config.rs)
- **InstallConfig** struct: holds all installation parameters
  - project_path: PathBuf
  - binary_name: Option<String>
  - use_debug: bool
  - verbose: bool
  - dry_run: bool

- Methods:
  - new() - construct from CLI args
  - destination_dir() - compute ~/.local/softwarewrighter/bin
  - source_binary_path() - compute target/release/<name> or target/debug/<name>
  - destination_binary_path() - compute destination file path

#### Validation (validator.rs)
- **Validator** struct: validates installation prerequisites

- Methods:
  - validate_project_path() - ensure path exists and is directory
  - validate_cargo_toml() - ensure Cargo.toml exists
  - validate_source_binary() - ensure compiled binary exists
  - get_binary_name() - extract binary name from Cargo.toml

#### Installation (installer.rs)
- **Installer** struct: performs the installation

- Methods:
  - install() - main installation orchestration
  - create_destination_dir() - mkdir -p equivalent
  - copy_binary() - copy file from source to destination
  - set_permissions() - chmod +x on Unix systems

#### Uninstallation (uninstaller.rs)
- **Uninstaller** struct: performs binary removal

- Methods:
  - uninstall() - main uninstallation orchestration
  - validate_binary_exists() - verify binary is installed
  - remove_binary() - delete file from installation directory

#### Output (output.rs)
- **OutputHandler** trait: abstraction for output modes

- Implementations:
  - NormalOutput - minimal output
  - VerboseOutput - detailed step-by-step output
  - DryRunOutput - print actions without executing

### Data Flow

1. User invokes CLI: `sw-install -p ../ask -v`
2. Clap parses arguments into Args struct
3. InstallConfig built from Args
4. Validator checks:
   - Project path exists
   - Cargo.toml exists at project_path/Cargo.toml
   - Binary exists at project_path/target/release/<name>
5. Installer performs installation:
   - Create ~/.local/softwarewrighter/bin/ if needed
   - Copy binary to destination
   - Set executable bit
6. Output handler prints results

### Error Handling
- Use Result<T, InstallError> throughout
- InstallError enum covers all error cases:
  - ProjectNotFound
  - CargoTomlNotFound
  - BinaryNotFound
  - IoError(std::io::Error)
  - PermissionError

- Errors bubble up to main() for user-friendly display

### Dependencies
- **clap**: CLI argument parsing with derive macros
- **toml**: Parse Cargo.toml to extract binary name
- **anyhow**: Enhanced error handling
- **std::fs**: File system operations
- **std::path**: Path manipulation

### Testing Strategy
- **Unit tests**: Test each component in isolation
  - Config path computations
  - Validator logic with mock filesystems
  - Installer operations with temp directories

- **Integration tests**: Test full workflows
  - tests/integration_test.rs
  - Create temporary Cargo projects
  - Verify end-to-end installation

- **Property-based tests**: (future)
  - Valid path transformations
  - Permission preservation

### Platform Considerations
- **Target platform**: Unix-like systems (macOS, Linux)
- **File paths**: Use PathBuf for cross-platform compatibility
- **Permissions**: Unix-specific chmod operations
- **Home directory**: Use std::env::var("HOME") with fallback

### Security Considerations
- Validate all user inputs (paths, names)
- Prevent path traversal attacks
- Check file permissions before overwriting
- No privilege escalation required
- Operate only in user-owned directories
