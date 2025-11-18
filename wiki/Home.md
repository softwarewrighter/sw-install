# sw-install Wiki

Welcome to the **sw-install** wiki! This is a comprehensive guide to the architecture, design, and implementation of sw-install - a command-line tool for installing Rust binaries to your local PATH.

## Quick Links

### Architecture Documentation
- **[Architecture Overview](Architecture-Overview)** - High-level system architecture and design principles
- **[Architecture Diagrams](Architecture-Diagrams)** - Visual block diagrams of the system
- **[Sequence Diagrams](Sequence-Diagrams)** - Execution flow diagrams for key operations
- **[Data Flow](Data-Flow)** - How data moves through the system

### Component Documentation
- **[CLI Interface](CLI-Interface)** - Command-line argument parsing and entry point
- **[Configuration](Configuration)** - Configuration management and path resolution
- **[Validator](Validator)** - Project and binary validation logic
- **[Installer](Installer)** - Binary installation operations
- **[Uninstaller](Uninstaller)** - Binary removal operations
- **[Setup](Setup)** - First-time installation directory setup
- **[Output Handler](Output-Handler)** - Output abstraction for different modes
- **[Error Handling](Error-Handling)** - Error types and handling strategy

### Development Guides
- **[Testing Strategy](Testing-Strategy)** - Unit, integration, and property-based testing
- **[Build System](Build-System)** - Build configuration and metadata capture

## What is sw-install?

`sw-install` is a command-line tool that simplifies the installation of compiled Rust binaries from local Cargo projects into `~/.local/softwarewrighter/bin/`, making them easily accessible from your PATH.

### Key Features
- Install release or debug builds
- Rename binaries during installation
- List installed binaries with timestamps
- Uninstall installed binaries
- Dry-run mode to preview actions
- Verbose output for debugging
- First-time setup with PATH configuration

## Architecture Principles

### Separation of Concerns
The application is organized into distinct modules, each with a single responsibility:
- **CLI Interface**: Argument parsing only
- **Configuration**: Path logic and settings
- **Validation**: Pre-installation checks
- **Installation**: File operations
- **Output**: Display logic abstraction

### Error Handling
All operations return `Result<T, InstallError>` types, ensuring errors are handled explicitly and bubble up with context.

### Testability
Components are designed with dependency injection, allowing for comprehensive unit and integration testing.

### Platform Compatibility
Uses `PathBuf` for cross-platform path handling while targeting Unix-like systems (macOS, Linux).

## Getting Started

For usage instructions, see the main [README](https://github.com/softwarewrighter/sw-install/blob/main/README.md).

For architecture details, start with the [Architecture Overview](Architecture-Overview).

For understanding execution flow, see the [Sequence Diagrams](Sequence-Diagrams).

## Contributing

This project follows Test-Driven Development (TDD) methodology. See the [Testing Strategy](Testing-Strategy) page for details on how to write tests for new features.
