# sw-install

A command-line tool for installing softwarewrighter Rust binaries to your local PATH.

## Overview

`sw-install` simplifies the installation of compiled Rust binaries from local Cargo projects into `~/.local/softwarewrighter/bin/`, making them easily accessible from your PATH.

## Features

- **Setup Installation Directory**: One-time setup to create install directory and configure PATH
- Install release or debug builds
- Rename binaries during installation to avoid conflicts
- Uninstall installed binaries
- Dry-run mode to preview actions
- Verbose output for detailed step-by-step information
- Test mode for development and testing

## Installation

### Build from Source

```bash
git clone https://github.com/softwarewrighter/sw-install.git
cd sw-install
cargo build --release
```

The binary will be available at `target/release/sw-install`.

### First-Time Setup

Run the setup command to create the installation directory and configure your PATH:

```bash
./target/release/sw-install --setup-install-dir
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

### Uninstall a Binary

```bash
sw-install -u my-binary-name
```

### Preview Actions (Dry-Run)

See what would happen without making changes:

```bash
sw-install -p ~/projects/my-tool --dry-run --verbose
```

### Verbose Output

Get detailed step-by-step output:

```bash
sw-install -p ~/projects/my-tool --verbose
```

### Version Information

Display version, build information, copyright, and repository details:

```bash
sw-install --version
# or
sw-install -V
```

This shows:
- Version number
- Copyright and license information
- Repository URL
- Build host
- Git commit SHA
- ISO 8601 build timestamp

### Testing Mode

For development and testing, override the destination directory:

```bash
sw-install -p ~/projects/my-tool --test-dir /tmp/test-bin
```

## Command-Line Options

```
Options:
  -p, --project <PATH>          Path to the Cargo project (for installation)
  -r, --rename <NAME>           Rename the binary during installation
      --type <TYPE>             Build type to install (release or debug) [default: release]
  -u, --uninstall <NAME>        Uninstall the named binary
  -s, --setup-install-dir       Setup installation directory and configure PATH
  -v, --verbose                 Show verbose output
  -n, --dry-run                 Print actions without executing them
  -t, --test-dir <DIR>          Override destination directory for testing
  -h, --help                    Print help (use --help for extended help)
  -V, --version                 Print version
```

## Extended Help

For comprehensive documentation including AI agent guidance and workflow details:

```bash
sw-install --help
```

## Prerequisites

- Run `sw-install --setup-install-dir` for first-time setup
- The project must have a `Cargo.toml` file
- The binary must be compiled before installation:
  - For release: `cargo build --release`
  - For debug: `cargo build`

## Examples

### First-Time Usage

1. Set up the installation directory:
   ```bash
   sw-install --setup-install-dir
   source ~/.bashrc  # or ~/.zshrc
   ```

2. Build your project:
   ```bash
   cd ~/projects/my-cli-tool
   cargo build --release
   ```

3. Install it:
   ```bash
   sw-install -p ~/projects/my-cli-tool
   ```

4. Use it:
   ```bash
   my-cli-tool --help
   ```

### Development Workflow

1. Make changes to your code
2. Build debug version:
   ```bash
   cargo build
   ```

3. Install debug version with different name:
   ```bash
   sw-install -p . --type debug --rename my-tool-dev
   ```

4. Test your changes:
   ```bash
   my-tool-dev test-command
   ```

### Uninstallation

```bash
sw-install -u my-tool
```

Preview uninstallation:

```bash
sw-install -u my-tool --dry-run --verbose
```

## Development

### Running Tests

```bash
cargo test
```

### Pre-commit Checks

Before committing, run the quality checks:

```bash
./scripts/pre-commit-check.sh
```

This script:
- Formats code with `cargo fmt`
- Checks for clippy warnings
- Builds the project
- Runs all tests
- Validates .gitignore
- Checks documentation encoding

### Code Quality

- All code must pass `cargo clippy` with zero warnings
- Code must be formatted with `cargo fmt`
- All tests must pass
- Documentation must be ASCII-only UTF-8

## Documentation

- [PRD](docs/prd.md) - Product Requirements Document
- [Architecture](docs/architecture.md) - System architecture
- [Design](docs/design.md) - Detailed design documentation
- [Process](docs/process.md) - Development process and workflow
- [Plan](docs/plan.md) - Implementation plan
- [Status](docs/status.md) - Current project status

## License

MIT License - Copyright (c) 2025 Michael A Wright

See [LICENSE](LICENSE) for details.

## Contributing

This project follows Test-Driven Development (TDD) methodology. All contributions should:

1. Include tests for new functionality
2. Pass all pre-commit checks
3. Follow the Rust style guide
4. Maintain zero clippy warnings
5. Update documentation as needed

## Troubleshooting

### Binary not found after installation

Run the setup command if you haven't already:

```bash
sw-install --setup-install-dir
source ~/.bashrc  # or ~/.zshrc
```

Verify PATH is configured:

```bash
echo $PATH | grep softwarewrighter
```

### Source binary not found

The error "Source binary not found" means the binary hasn't been compiled yet. Run:

```bash
cargo build --release
```

Or for debug builds:

```bash
cargo build
```

### Permission errors

sw-install operates only in user-owned directories and doesn't require sudo or elevated privileges.

## AI Agent Guidance

This tool is designed for automated binary installation in development workflows:

- Use `--dry-run (-n)` to preview actions before execution
- Use `--verbose (-v)` to see detailed step-by-step output
- Check exit codes: 0 = success, non-zero = error
- Combine flags: `-nvp` for verbose dry-run installation
- All file paths are validated before operations
- Errors include actionable suggestions

## Security

- Operates only in user-owned directories
- No privilege escalation required
- Validates all paths to prevent traversal attacks
- Safe to run in automated environments
