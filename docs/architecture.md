# Architecture

## sw-install

### Overview
sw-install is a command-line tool that installs compiled Rust binaries from local Cargo projects into a user-specific installation directory.

### Multi-Component Architecture

The project is organized as a multi-component structure with 7 independent crates, each with its own Cargo.toml and target directory. This follows sw-standards for cognitive load management (Miller's Law).

```
sw-install/
├── components/
│   ├── sw-install-core/        # Core types: config, output, errors, utilities
│   ├── sw-install-workspace/   # Cargo workspace utilities
│   ├── sw-install-validation/  # Project validation and binary detection
│   ├── sw-install-installer/   # Install and uninstall operations
│   ├── sw-install-manage/      # Setup operations
│   ├── sw-install-list/        # List installed binaries
│   └── sw-install-cli/         # CLI binary (main entry point)
├── scripts/
│   └── build.sh                # Builds all components in dependency order
└── docs/
```

### Component Dependency Graph

```
                    sw-install-cli
                    /     |     \     \
                   /      |      \     \
                  v       v       v     v
        sw-install-   sw-install-  sw-install-  sw-install-
         validation    installer     manage       list
              |            |           |            |
              v            v           v            v
        sw-install-   sw-install-  sw-install-  sw-install-
         workspace       core        core         core
              |
              v
        sw-install-core
```

### Component Details

#### sw-install-core (4 modules)
Core types shared across all components:
- `config.rs` - InstallConfig struct with path computations
- `output.rs` - NormalOutput for user feedback
- `format.rs` - Time formatting utilities (format_time_ago)
- `lib.rs` - InstallError enum, Result type, re-exports

#### sw-install-workspace (1 module)
Cargo workspace utilities:
- `lib.rs` - find_workspace_binaries, expand_member_paths, extract_binaries_from_member

#### sw-install-validation (4 modules)
Project validation and binary detection:
- `detect.rs` - Project type detection (Simple, Workspace, MultiComponent)
- `extract.rs` - Binary name extraction from Cargo.toml
- `source.rs` - Source binary validation and freshness checking
- `lib.rs` - Validator struct, ValidationResult

#### sw-install-installer (4 modules)
Install and uninstall operations:
- `install.rs` - Installer struct for binary installation
- `uninstall.rs` - Uninstaller struct for binary removal
- `paths.rs` - Path utilities for destination directories
- `lib.rs` - Re-exports

#### sw-install-manage (3 modules)
Setup operations:
- `setup.rs` - Setup struct for first-time configuration
- `shell.rs` - Shell configuration detection and PATH setup
- `lib.rs` - Re-exports

#### sw-install-list (4 modules)
List installed binaries:
- `list.rs` - Lister struct for listing binaries
- `binaries.rs` - Binary collection and directory utilities
- `sort.rs` - SortOrder enum and parsing
- `lib.rs` - Re-exports

#### sw-install-cli (5 modules)
CLI binary entry point:
- `main.rs` - Entry point, CLI parsing, dispatch
- `install.rs` - Install command handler
- `manage.rs` - Setup, list, uninstall command handlers
- `version.rs` - Version information display
- `help.txt` - Extended help text

### High-Level Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                   CLI Interface (sw-install-cli)                │
│              (clap argument parser + dispatch)                  │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│               Configuration Layer (sw-install-core)             │
│  - Parse arguments                                              │
│  - Build InstallConfig                                          │
│  - Create NormalOutput                                          │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│            Validation Layer (sw-install-validation)             │
│  - Detect project type (Simple/Workspace/MultiComponent)        │
│  - Extract binary name from Cargo.toml                          │
│  - Verify source binary exists and is fresh                     │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│            Installation Layer (sw-install-installer)            │
│  - Create destination directory                                 │
│  - Copy binary                                                  │
│  - Set executable permissions                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Project Type Detection

The validator automatically detects three project structures:

1. **Simple Project** - Standard Cargo project with root Cargo.toml and package section
2. **Workspace Project** - Cargo workspace with members array, auto-detects binaries
3. **Multi-Component** - No root Cargo.toml, scans components/ directory for workspace crates

### Error Handling
- Use `Result<T, InstallError>` throughout
- InstallError enum covers all error cases:
  - ProjectNotFound, NotADirectory
  - CargoTomlNotFound, CargoTomlParse
  - BinaryNameNotFound, BinaryNotFound, BinaryOutdated
  - BinaryNotInstalled, InstallDirNotFound
  - InvalidBinaryName, HomeNotFound
  - NoOperationSpecified, Io

### sw-standards Compliance

The codebase follows sw-checklist standards:
- Functions per module: 4 or fewer (warning threshold)
- Modules per crate: 4 or fewer (warning threshold)
- Lines per function: 25 or fewer (warning threshold)
- Components: Independent crates with own Cargo.toml

Current status: **45 checks passed, 0 failed, 0 code quality warnings**

### Dependencies

**Runtime:**
- clap - CLI argument parsing with derive macros
- toml - Parse Cargo.toml to extract binary name
- thiserror - Error handling

**Dev:**
- tempfile - Temporary directories for tests
- serial_test - Test isolation

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
