# Testing Strategy

This page describes the comprehensive testing strategy for sw-install, following Test-Driven Development (TDD) principles.

## Testing Pyramid

```
         ┌─────────────────┐
         │   Integration   │  ← End-to-end workflows
         │     Tests       │
         └─────────────────┘
                ▲
         ┌──────────────────┐
         │   Component      │  ← Module interactions
         │     Tests        │
         └──────────────────┘
                ▲
         ┌───────────────────┐
         │    Unit Tests     │  ← Individual functions
         │                   │
         └───────────────────┘
```

## Test Levels

### 1. Unit Tests

Test individual functions and methods in isolation.

**Location**: Within each module (`#[cfg(test)] mod tests`)

**Coverage:**
- `config.rs`: Path computations
- `validator.rs`: Validation logic
- `installer.rs`: File operations
- `uninstaller.rs`: Removal logic
- `output.rs`: Output formatting
- `error.rs`: Error messages

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_dir() {
        let config = InstallConfig::new(
            PathBuf::from("/test"),
            None, false, false, false, None,
        );

        let dest = config.destination_dir().unwrap();
        assert!(dest.ends_with(".local/softwarewrighter/bin"));
    }
}
```

### 2. Integration Tests

Test complete workflows across multiple modules.

**Location**: `tests/integration.rs`

**Coverage:**
- Full installation workflow
- Uninstall workflow
- List workflow
- Setup workflow
- Error scenarios

**Example:**
```rust
#[test]
fn test_install_workflow() {
    let temp_dir = create_test_project();

    // Run installation
    let result = run_install(&temp_dir);
    assert!(result.is_ok());

    // Verify binary exists
    let binary = get_install_path("test-binary");
    assert!(binary.exists());

    // Verify permissions
    #[cfg(unix)]
    {
        let metadata = fs::metadata(&binary).unwrap();
        let permissions = metadata.permissions();
        assert!(permissions.mode() & 0o111 != 0); // executable
    }
}
```

### 3. Command-Line Tests

Test CLI argument parsing and error messages.

**Location**: `tests/cli.rs`

**Coverage:**
- Argument validation
- Help text
- Version output
- Error messages

**Example:**
```rust
#[test]
fn test_no_operation_error() {
    let output = Command::new("sw-install")
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("No operation specified"));
}
```

## Test Utilities

### Test Output Handler

```rust
#[cfg(test)]
pub struct TestOutput {
    messages: Arc<Mutex<Vec<String>>>,
}

impl TestOutput {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }

    pub fn assert_contains(&self, substring: &str) {
        let messages = self.get_messages();
        assert!(
            messages.iter().any(|m| m.contains(substring)),
            "No message contains: {}",
            substring
        );
    }
}
```

### Test Project Creation

```rust
#[cfg(test)]
fn create_test_project(name: &str) -> TempDir {
    let temp_dir = tempdir().unwrap();

    // Create Cargo.toml
    let cargo_toml = temp_dir.path().join("Cargo.toml");
    fs::write(&cargo_toml, format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n",
        name
    )).unwrap();

    // Create target directory
    let target_dir = temp_dir.path().join("target/release");
    fs::create_dir_all(&target_dir).unwrap();

    // Create fake binary
    let binary = target_dir.join(name);
    fs::write(&binary, "fake binary content").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&binary, perms).unwrap();
    }

    temp_dir
}
```

## Test Categories

### Happy Path Tests

Test successful operations:

```rust
#[test]
fn test_successful_install() {
    // Setup
    let project = create_test_project("my-tool");

    // Execute
    let result = install(&project);

    // Assert
    assert!(result.is_ok());
}
```

### Error Path Tests

Test error handling:

```rust
#[test]
fn test_missing_project() {
    let config = InstallConfig::new(
        PathBuf::from("/nonexistent"),
        None, false, false, false, None,
    );

    let result = Validator::new(&config, test_output()).validate();

    assert!(matches!(
        result,
        Err(InstallError::ProjectNotFound(_))
    ));
}
```

### Edge Case Tests

Test boundary conditions:

```rust
#[test]
fn test_empty_binary_name() {
    let error = InstallError::InvalidBinaryName("".to_string());
    assert!(format!("{}", error).contains("Invalid binary name"));
}

#[test]
fn test_rename_same_as_original() {
    // Should work fine
    let config = InstallConfig::new(
        project_path,
        Some("my-tool".to_string()), // Same as original
        false, false, false, None,
    );
    // ...
}
```

### Regression Tests

Test previously fixed bugs:

```rust
#[test]
fn test_no_operation_error_includes_all_options() {
    // Regression: Error message was incomplete
    let error = InstallError::NoOperationSpecified;
    let msg = format!("{}", error);

    assert!(msg.contains("--project"));
    assert!(msg.contains("--uninstall"));
    assert!(msg.contains("--list"));
    assert!(msg.contains("--setup-install-dir"));
}
```

## Test Organization

### Module Structure

```
tests/
├── integration.rs       # Full workflow tests
├── cli.rs              # CLI argument tests
├── error_handling.rs   # Error scenario tests
└── common/
    ├── mod.rs
    ├── fixtures.rs     # Test data
    └── helpers.rs      # Test utilities
```

### Test Naming Convention

```rust
// Format: test_<what>_<scenario>
#[test]
fn test_install_with_rename() { }

#[test]
fn test_validate_missing_cargo_toml() { }

#[test]
fn test_list_sorts_by_newest() { }
```

## Coverage Goals

### Target Coverage

- **Unit tests**: 90%+ line coverage
- **Integration tests**: All major workflows
- **Error paths**: All error variants

### Running Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

## Test Execution

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_install_workflow
```

### Run Integration Tests Only

```bash
cargo test --test integration
```

### Run with Output

```bash
cargo test -- --nocapture
```

### Run Ignored Tests

```bash
cargo test -- --ignored
```

## Test-Driven Development Process

### 1. Red Phase

Write failing test first:

```rust
#[test]
fn test_list_sorts_by_newest() {
    let lister = Lister::new(SortOrder::Newest, false, test_output());
    let result = lister.list().unwrap();

    // This will fail until implemented
    assert_eq!(result[0].name, "newest-binary");
}
```

### 2. Green Phase

Implement minimal code to pass:

```rust
impl Lister {
    pub fn list(&self) -> Result<Vec<BinaryInfo>> {
        let mut binaries = self.scan_directory()?;

        if self.sort_order == SortOrder::Newest {
            binaries.sort_by(|a, b| b.modified.cmp(&a.modified));
        }

        Ok(binaries)
    }
}
```

### 3. Refactor Phase

Improve code while keeping tests green:

```rust
impl Lister {
    pub fn list(&self) -> Result<Vec<BinaryInfo>> {
        let mut binaries = self.scan_directory()?;
        self.sort_binaries(&mut binaries);
        Ok(binaries)
    }

    fn sort_binaries(&self, binaries: &mut Vec<BinaryInfo>) {
        match self.sort_order {
            SortOrder::Name => binaries.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOrder::Newest => binaries.sort_by(|a, b| b.modified.cmp(&a.modified)),
            SortOrder::Oldest => binaries.sort_by(|a, b| a.modified.cmp(&b.modified)),
        }
    }
}
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Run tests
      run: cargo test --verbose

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Check formatting
      run: cargo fmt -- --check
```

## Test Best Practices

### 1. Arrange-Act-Assert (AAA)

```rust
#[test]
fn test_example() {
    // Arrange: Set up test data
    let config = create_test_config();

    // Act: Execute operation
    let result = operation(&config);

    // Assert: Verify outcome
    assert!(result.is_ok());
}
```

### 2. One Assertion Per Test

```rust
// Good: Focused test
#[test]
fn test_config_destination_dir() {
    let config = create_test_config();
    let dest = config.destination_dir().unwrap();
    assert!(dest.ends_with("softwarewrighter/bin"));
}

// Avoid: Multiple unrelated assertions
#[test]
fn test_config() {
    let config = create_test_config();
    assert!(config.destination_dir().is_ok());
    assert!(config.source_binary_path("test").exists());
    assert!(config.verbose == false);
}
```

### 3. Use Descriptive Names

```rust
// Good
#[test]
fn test_install_creates_executable_binary() { }

// Poor
#[test]
fn test1() { }
```

### 4. Clean Up Resources

```rust
#[test]
fn test_with_cleanup() {
    let temp_dir = tempdir().unwrap();

    // Test code

    // Cleanup happens automatically when temp_dir is dropped
}
```

### 5. Test Error Messages

```rust
#[test]
fn test_error_message_includes_path() {
    let path = PathBuf::from("/test/path");
    let error = InstallError::ProjectNotFound(path);

    let message = format!("{}", error);
    assert!(message.contains("/test/path"));
}
```

## Debugging Tests

### Print Debug Output

```rust
#[test]
fn test_with_debug() {
    let result = operation();
    println!("Result: {:?}", result); // Only shows with --nocapture
    assert!(result.is_ok());
}
```

### Use dbg! Macro

```rust
#[test]
fn test_with_dbg() {
    let config = create_config();
    dbg!(&config); // Prints to stderr
    // ...
}
```

## See Also

- [CLI Interface](CLI-Interface) - CLI testing
- [Validator](Validator) - Validation testing
- [Installer](Installer) - Installation testing
- [Error Handling](Error-Handling) - Error testing
