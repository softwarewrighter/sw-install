# Architecture Overview

## System Overview

`sw-install` is designed as a layered application with clear separation between CLI parsing, validation, business logic, and file operations.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                      │
│              (clap argument parser)                  │
│         • Parse command-line arguments               │
│         • Version information display                │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│               Configuration Layer                    │
│  • Parse arguments                                   │
│  • Build InstallConfig                               │
│  • Path resolution logic                             │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│              Validation Layer                        │
│  • Verify project path exists                        │
│  • Verify Cargo.toml exists                          │
│  • Extract binary name from Cargo.toml               │
│  • Verify binary exists in target/                   │
└────────────────┬────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│              Operation Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │  Installer   │  │ Uninstaller  │  │  Lister   │ │
│  │              │  │              │  │           │ │
│  │ • Create dir │  │ • Validate   │  │ • Scan    │ │
│  │ • Copy file  │  │ • Remove bin │  │ • Format  │ │
│  │ • Set perms  │  │              │  │ • Sort    │ │
│  └──────────────┘  └──────────────┘  └───────────┘ │
└─────────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────┐
│              Output Handler Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │   Normal     │  │   Verbose    │  │  DryRun   │ │
│  │   Output     │  │   Output     │  │  Output   │ │
│  └──────────────┘  └──────────────┘  └───────────┘ │
└─────────────────────────────────────────────────────┘
```

## Component Responsibilities

### CLI Interface (`main.rs`)
- **Purpose**: Entry point for the application
- **Responsibilities**:
  - Parse command-line arguments using clap
  - Display version information
  - Route to appropriate operation (install, uninstall, list, setup)
  - Handle top-level error display

### Configuration (`config.rs`)
- **Purpose**: Centralize configuration and path logic
- **Responsibilities**:
  - Store installation parameters
  - Compute source binary paths
  - Compute destination paths
  - Handle rename logic

### Validator (`validator.rs`)
- **Purpose**: Pre-flight validation before operations
- **Responsibilities**:
  - Verify project directory exists
  - Verify Cargo.toml exists
  - Parse Cargo.toml to extract binary name
  - Verify compiled binary exists

### Installer (`installer.rs`)
- **Purpose**: Perform binary installation
- **Responsibilities**:
  - Create destination directory if needed
  - Copy binary from source to destination
  - Set executable permissions (Unix)
  - Report success/failure

### Uninstaller (`uninstaller.rs`)
- **Purpose**: Remove installed binaries
- **Responsibilities**:
  - Verify binary exists in install directory
  - Remove binary file
  - Report success/failure

### Lister (`lister.rs`)
- **Purpose**: Display installed binaries
- **Responsibilities**:
  - Scan installation directory
  - Retrieve file metadata (timestamps)
  - Format human-readable time strings
  - Sort by name, newest, or oldest

### Setup (`setup.rs`)
- **Purpose**: First-time installation setup
- **Responsibilities**:
  - Create installation directory
  - Detect shell type (bash/zsh)
  - Update shell configuration file
  - Provide instructions to user

### Output Handler (`output.rs`)
- **Purpose**: Abstract output display logic
- **Responsibilities**:
  - Provide trait for different output modes
  - Implement normal, verbose, and dry-run outputs
  - Consistent formatting across operations

### Error Handling (`error.rs`)
- **Purpose**: Centralize error types and messages
- **Responsibilities**:
  - Define all error variants
  - Provide user-friendly error messages
  - Enable error propagation with `?` operator

## Design Patterns

### Strategy Pattern
The `OutputHandler` trait implements the Strategy pattern, allowing the output mode to be selected at runtime without changing the business logic.

### Dependency Injection
Components receive their dependencies through constructors, improving testability:
```rust
Installer::new(config, binary_name, output)
Validator::new(config, output)
```

### Builder Pattern
Configuration is constructed from CLI arguments in a structured way:
```rust
InstallConfig::new(project_path, binary_name, use_debug, verbose, dry_run)
```

### Result-Based Error Handling
All fallible operations return `Result<T, InstallError>`, forcing explicit error handling.

## Data Flow

1. **CLI Input** → Arguments parsed by clap
2. **Configuration** → InstallConfig built from arguments
3. **Validation** → Validator checks preconditions
4. **Operation** → Installer/Uninstaller/Lister performs action
5. **Output** → OutputHandler displays results

## Key Design Decisions

### Single Responsibility
Each module has one clear purpose, making the code easier to understand and test.

### Fail Fast
Validation happens before any file operations, preventing partial installations.

### Testable Design
Dependency injection and trait abstractions enable comprehensive testing.

### User Experience
- Dry-run mode lets users preview actions
- Verbose mode provides debugging information
- Clear error messages with actionable suggestions

### Security
- All paths validated before use
- No privilege escalation required
- Operates only in user-owned directories

## Technology Stack

### Core Dependencies
- **clap** - CLI argument parsing with derive macros
- **toml** - Parse Cargo.toml files
- **thiserror** - Error type definitions
- **chrono** - Timestamp formatting for list command

### Standard Library
- **std::fs** - File system operations
- **std::path** - Path manipulation
- **std::env** - Environment variables

## Platform Support

**Target Platforms**: Unix-like systems (macOS, Linux)

**Platform-Specific Features**:
- File permissions (Unix chmod)
- Home directory resolution
- Shell configuration (bash/zsh)

## See Also

- [Architecture Diagrams](Architecture-Diagrams) - Visual representations
- [Sequence Diagrams](Sequence-Diagrams) - Execution flows
- [Component Documentation](Home#component-documentation) - Detailed component docs
