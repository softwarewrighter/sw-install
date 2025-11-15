# Product Requirements Document (PRD)

## sw-install

### Overview
A Rust CLI tool that installs compiled binaries from Cargo projects into a user's local binary directory, making them accessible from PATH.

### Problem Statement
Developers working with multiple CLI tools in the softwarewrighter organization need an easy way to:
- Install binaries to a consistent location (~/.local/softwarewrighter/bin)
- Avoid naming conflicts with existing PATH binaries
- Switch between debug and release builds during development
- Understand what the installer is doing (verbose mode)
- Test installation without making changes (dry-run mode)

### Target Users
- Software developers using softwarewrighter CLI tools
- Developers who prefer local installations over system-wide installations
- Teams needing consistent binary installation across development environments

### Requirements

#### Functional Requirements

##### FR1: Project Installation
- **FR1.1**: Install a binary from a Cargo project to ~/.local/softwarewrighter/bin/
- **FR1.2**: Support both release and debug builds
- **FR1.3**: Create necessary directories if they don't exist
- **FR1.4**: Copy the compiled binary to the installation directory

##### FR2: Command-Line Interface
- **FR2.1**: Accept `-p/--project <path>` to specify the source project path
- **FR2.2**: Accept `-r/--rename <name>` to rename the binary during installation
- **FR2.3**: Accept `--type [release|debug]` option to specify build type (default: release)
- **FR2.4**: Accept `-v/--verbose` flag to show detailed installation steps
- **FR2.5**: Accept `-n/--dry-run` flag to print actions without executing them
- **FR2.6**: Accept `-u/--uninstall <name>` to remove a binary from the installation directory
- **FR2.7**: Provide extended help with `--help` for AI agents and tool-using LLMs

##### FR3: Validation
- **FR3.1**: Verify the project path exists and contains a Cargo.toml
- **FR3.2**: Verify the binary exists in target/release or target/debug
- **FR3.3**: Provide clear error messages for validation failures

##### FR4: Uninstallation
- **FR4.1**: Remove specified binary from ~/.local/softwarewrighter/bin/
- **FR4.2**: Verify binary exists before attempting removal
- **FR4.3**: Support dry-run and verbose modes for uninstallation
- **FR4.4**: Provide clear feedback on uninstallation success or failure

##### FR5: Output Modes
- **FR5.1**: Normal mode: minimal output showing success or failure
- **FR5.2**: Verbose mode: show each step (validation, copy, permissions)
- **FR5.3**: Dry-run mode: print all actions without executing them
- **FR5.4**: Extended help mode: comprehensive documentation for AI agents

#### Non-Functional Requirements

##### NFR1: Code Quality
- **NFR1.1**: All code must pass `cargo fmt` formatting checks
- **NFR1.2**: All code must pass `cargo clippy` with zero warnings
- **NFR1.3**: Fix root causes of warnings; do not disable lints
- **NFR1.4**: Achieve 100% test pass rate
- **NFR1.5**: Follow Test-Driven Development (TDD) methodology

##### NFR2: Documentation
- **NFR2.1**: All markdown files must be UTF-8 encoded (ASCII subset)
- **NFR2.2**: No unprintable characters (e.g., tree branch symbols)
- **NFR2.3**: Maintain comprehensive documentation in docs/
- **NFR2.4**: Keep documentation synchronized with code changes

##### NFR3: Development Process
- **NFR3.1**: Follow pre-commit validation workflow
- **NFR3.2**: Format source before committing
- **NFR3.3**: Ensure .gitignore is up-to-date

### Success Metrics
- Installation completes in < 5 seconds for typical projects
- Zero clippy warnings in production code
- 100% test coverage for core functionality
- Clear, actionable error messages for all failure scenarios

### Out of Scope
- Installing from remote repositories (git URLs)
- Managing multiple versions of the same binary
- Automatic PATH configuration
- Windows or non-Unix platform support (initial version)

### Future Considerations
- List installed binaries
- Update check for installed binaries
- Cross-platform support (Windows, macOS, Linux)
- Configuration file for default settings
- Dependency tracking for installed binaries
