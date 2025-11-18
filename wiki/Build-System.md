# Build System

This page describes the build configuration and metadata capture for sw-install.

## Build Script

The `build.rs` script captures build-time metadata that is embedded into the binary.

### Implementation

```rust
// build.rs
use std::process::Command;

fn main() {
    // Capture hostname (build host)
    let hostname = Command::new("hostname")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_HOST={}", hostname);

    // Capture git commit SHA
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Capture build timestamp in ISO 8601 format
    let timestamp = Command::new("date")
        .args(["+%Y-%m-%dT%H:%M:%S%z"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Rebuild if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
```

## Captured Metadata

### 1. Build Host

**Environment Variable**: `BUILD_HOST`

**Captures**: Hostname of the machine where the build occurred

**Command**: `hostname`

**Example**: `"manager"`, `"build-server"`

### 2. Git Commit

**Environment Variable**: `GIT_HASH`

**Captures**: Short SHA of the git commit

**Command**: `git rev-parse --short HEAD`

**Example**: `"c661d31"`, `"a290a38"`

### 3. Build Timestamp

**Environment Variable**: `BUILD_TIMESTAMP`

**Captures**: ISO 8601 formatted timestamp

**Command**: `date +%Y-%m-%dT%H:%M:%S%z`

**Format**: `YYYY-MM-DDTHH:MM:SS±ZZZZ`

**Example**: `"2025-01-17T09:15:38-0800"`

## Accessing Metadata at Runtime

The captured environment variables are available via `env!` macro:

```rust
const BUILD_HOST: &str = env!("BUILD_HOST");
const GIT_HASH: &str = env!("GIT_HASH");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
```

## Version Information Display

### Implementation

```rust
const REPOSITORY: &str = "https://github.com/softwarewrighter/sw-install";
const LICENSE: &str = "MIT";
const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";

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

## Standard Cargo Metadata

Cargo provides several built-in environment variables:

### Package Information

```rust
env!("CARGO_PKG_NAME")       // "sw-install"
env!("CARGO_PKG_VERSION")    // "0.1.0"
env!("CARGO_PKG_AUTHORS")    // "Michael A Wright"
env!("CARGO_PKG_DESCRIPTION") // Package description
```

### Build Information

```rust
env!("CARGO_PKG_VERSION_MAJOR")  // "0"
env!("CARGO_PKG_VERSION_MINOR")  // "1"
env!("CARGO_PKG_VERSION_PATCH")  // "0"
```

## Cargo.toml Configuration

```toml
[package]
name = "sw-install"
version = "0.1.0"
edition = "2021"
authors = ["Michael A Wright"]
description = "Install softwarewrighter Rust binaries to local PATH"
license = "MIT"
repository = "https://github.com/softwarewrighter/sw-install"

[build-dependencies]
# None needed - uses standard library only

[dependencies]
clap = { version = "4.4", features = ["derive"] }
toml = "0.8"
thiserror = "1.0"
chrono = "0.4"

[dev-dependencies]
tempfile = "3.8"
```

## Build Triggers

### Rebuild Conditions

The build script specifies when to re-run:

```rust
println!("cargo:rerun-if-changed=.git/HEAD");
```

**Triggers rebuild when:**
- `.git/HEAD` changes (new commit, branch switch)
- `build.rs` itself changes
- Any source file changes (default Cargo behavior)

**Does NOT rebuild for:**
- Untracked file changes
- Stash operations
- Configuration file changes (unless specified)

## Release vs Debug Builds

### Debug Build

```bash
cargo build
```

**Characteristics:**
- Fast compilation
- Debug symbols included
- No optimizations
- Larger binary size
- Contains debug assertions

### Release Build

```bash
cargo build --release
```

**Characteristics:**
- Slower compilation (optimizations)
- Debug symbols stripped
- Full optimizations (opt-level = 3)
- Smaller binary size
- No debug assertions

## Build Profiles

### Default Profiles

```toml
[profile.dev]
opt-level = 0      # No optimizations
debug = true       # Include debug info

[profile.release]
opt-level = 3      # All optimizations
debug = false      # No debug info
lto = false        # Link-time optimization
```

### Custom Profile (Optional)

```toml
[profile.release-with-debug]
inherits = "release"
debug = true       # Release optimizations + debug symbols
```

## Build Output

### Debug Build

```
target/
└── debug/
    ├── sw-install           # Binary
    ├── sw-install.d         # Dependency info
    └── deps/                # Dependency artifacts
```

### Release Build

```
target/
└── release/
    ├── sw-install           # Optimized binary
    ├── sw-install.d         # Dependency info
    └── deps/                # Dependency artifacts
```

## Cross-Compilation

### Target Platforms

```bash
# macOS (x86_64)
cargo build --target x86_64-apple-darwin

# macOS (ARM64)
cargo build --target aarch64-apple-darwin

# Linux (x86_64)
cargo build --target x86_64-unknown-linux-gnu

# Linux (ARM64)
cargo build --target aarch64-unknown-linux-gnu
```

## Build Metadata Use Cases

### 1. Debugging

When users report issues, build metadata helps:
- Identify exact version (commit SHA)
- Know build environment (host, timestamp)
- Reproduce build conditions

### 2. Version Tracking

- Track which commit produced a binary
- Verify binary authenticity
- Audit trail for deployments

### 3. Support

- Quickly identify outdated builds
- Verify users are on correct version
- Provide precise troubleshooting

## Testing Build Metadata

```rust
#[test]
fn test_version_includes_metadata() {
    let version = format!(
        "{}",
        env!("CARGO_PKG_VERSION")
    );
    assert!(!version.is_empty());

    let build_host = env!("BUILD_HOST");
    assert!(!build_host.is_empty());

    let git_hash = env!("GIT_HASH");
    assert!(!git_hash.is_empty());
}
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Build release
  run: cargo build --release --verbose

- name: Check build metadata
  run: |
    ./target/release/sw-install --version
    ./target/release/sw-install --version | grep "Commit:"
    ./target/release/sw-install --version | grep "Timestamp:"
```

## Design Decisions

### Why build.rs Instead of Runtime Detection?

**build.rs approach:**
- Metadata frozen at build time
- No runtime overhead
- Reproducible builds
- No git dependency at runtime

**Runtime detection:**
- Requires git at runtime
- Overhead on every execution
- May not match actual build

**Choice**: build.rs for frozen, reliable metadata

### Why Short Git Hash?

- More readable than full SHA
- Sufficient for identification
- Common convention (git log --oneline)
- Saves space in output

## See Also

- [CLI Interface](CLI-Interface) - Version command
- [Testing Strategy](Testing-Strategy) - Testing build metadata
- [Architecture Overview](Architecture-Overview) - Build dependencies
