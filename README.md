# sw-install

A command-line tool for installing softwarewrighter Rust binaries to your local PATH.

## Overview

`sw-install` simplifies the installation of compiled Rust binaries from local Cargo projects into `~/.local/softwarewrighter/bin/`, making them easily accessible from your PATH.

## Features

- **Setup Installation Directory**: One-time setup to create install directory and configure PATH
- Install release or debug builds
- Rename binaries during installation to avoid conflicts
- **List installed binaries**: View all binaries with timestamps and sorting options
- Uninstall installed binaries
- Dry-run mode to preview actions
- Verbose output for detailed step-by-step information
- **Project type detection**:
  - Simple projects (standard Cargo)
  - Workspace projects (auto-detect binaries)
  - Multi-component projects (no root Cargo.toml)

## Installation

### Build from Source

```bash
git clone https://github.com/softwarewrighter/sw-install.git
cd sw-install
./scripts/build.sh
```

The binary will be available at `components/sw-install-cli/target/release/sw-install`.

### First-Time Setup

Run the setup command to create the installation directory and configure your PATH:

```bash
./components/sw-install-cli/target/release/sw-install --setup-install-dir
```

This will:
1. Create `~/.local/softwarewrighter/bin/` directory
2. Add PATH configuration to your shell config file (~/.zshrc or ~/.bashrc)
3. Show you how to reload your shell

After setup, reload your shell:

```bash
source ~/.bashrc  # or ~/.zshrc
```

## Usage

### Install a Binary

Install a release build:

```bash
sw-install -p ~/projects/my-cli-tool
```

Install a debug build:

```bash
sw-install -p ~/projects/my-cli-tool --type debug
```

Install with a different name:

```bash
sw-install -p ~/projects/ask --rename ask-dev
```

### List Installed Binaries

View all currently installed binaries with timestamps:

```bash
sw-install --list
```

Sort options:

```bash
sw-install --list --sort name     # alphabetical (default)
sw-install --list --sort newest   # most recently modified first
sw-install --list --sort oldest   # oldest first
```

### Uninstall a Binary

```bash
sw-install -u my-binary-name
```

### Preview Actions (Dry-Run)

See what would happen without making changes:

```bash
sw-install -p ~/projects/my-tool --dry-run --verbose
```

### Version Information

```bash
sw-install --version
```

## Command-Line Options

```
Options:
  -p, --project <PATH>          Path to the Cargo project (for installation)
  -r, --rename <NAME>           Rename the binary during installation
      --type <TYPE>             Build type to install (release or debug) [default: release]
  -u, --uninstall <NAME>        Uninstall the named binary
  -l, --list                    List all installed binaries
  -s, --sort <ORDER>            Sort order for list: name, oldest, newest [default: name]
      --setup-install-dir       Setup installation directory and configure PATH
  -v, --verbose                 Show verbose output
  -n, --dry-run                 Print actions without executing them
  -t, --test-dir <DIR>          Override destination directory for testing
  -h, --help                    Print help (use --help for extended help)
  -V, --version                 Print version
```

## Project Structure

sw-install uses a multi-component architecture with 7 independent crates:

```
sw-install/
├── components/
│   ├── sw-install-core/        # Config, output, errors, utilities
│   ├── sw-install-workspace/   # Cargo workspace utilities
│   ├── sw-install-validation/  # Project validation
│   ├── sw-install-installer/   # Install/uninstall operations
│   ├── sw-install-manage/      # Setup operations
│   ├── sw-install-list/        # List binaries
│   └── sw-install-cli/         # CLI binary
├── scripts/
│   └── build.sh                # Build all components
└── docs/                       # Documentation
```

## Development

### Building

Build all components:

```bash
./scripts/build.sh
```

Build a single component:

```bash
cd components/sw-install-cli
cargo build --release
```

### Running Tests

Tests are in the CLI component:

```bash
cd components/sw-install-cli
cargo test
```

### Code Quality

The project follows sw-standards (sw-checklist):

```bash
sw-checklist /path/to/sw-install
# Expected: 45 passed, 0 failed, 0 code quality warnings
```

Run pre-commit checks:

```bash
./scripts/pre-commit-check.sh
```

## Documentation

- [Architecture](docs/architecture.md) - Multi-component architecture
- [Design](docs/design.md) - Detailed design documentation
- [Plan](docs/plan.md) - Implementation plan
- [Status](docs/status.md) - Current project status
- [PRD](docs/prd.md) - Product Requirements Document

## Prerequisites

- Rust toolchain (for building)
- The target project must have a `Cargo.toml` file
- The binary must be compiled before installation

## Troubleshooting

### Binary not found after installation

Run the setup command:

```bash
sw-install --setup-install-dir
source ~/.bashrc  # or ~/.zshrc
```

### Source binary not found

The binary hasn't been compiled. Run:

```bash
cargo build --release
```

### Permission errors

sw-install operates only in user-owned directories and doesn't require elevated privileges.

## License

MIT License - Copyright (c) 2025 Michael A Wright

See [LICENSE](LICENSE) for details.

## AI Agent Guidance

This tool is designed for automated binary installation in development workflows:

- Use `--dry-run (-n)` to preview actions before execution
- Use `--verbose (-v)` to see detailed step-by-step output
- Check exit codes: 0 = success, non-zero = error
- All file paths are validated before operations
- Errors include actionable suggestions
