# Error Handling

The Error Handling module (`error.rs`) defines all error types and provides user-friendly error messages.

## Error Type

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Project path does not exist: {0}")]
    ProjectNotFound(PathBuf),

    #[error("Cargo.toml not found in project: {0}")]
    CargoTomlNotFound(PathBuf),

    #[error("Could not parse Cargo.toml: {0}")]
    CargoTomlParse(String),

    #[error("Binary name not found in Cargo.toml")]
    BinaryNameNotFound,

    #[error("Source binary not found: {0}\nHint: Run 'cargo build --release' in the project directory")]
    BinaryNotFound(PathBuf),

    #[error("Binary already exists in installation directory: {0}")]
    BinaryAlreadyExists(PathBuf),

    #[error("Binary not installed: {0}")]
    BinaryNotInstalled(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid binary name: {0}")]
    InvalidBinaryName(String),

    #[error("Home directory not found")]
    HomeNotFound,

    #[error("No operation specified. Use --project, --uninstall, --list, or --setup-install-dir")]
    NoOperationSpecified,

    #[error("Invalid sort order: {0}. Valid options: name, newest, oldest")]
    InvalidSortOrder(String),
}

pub type Result<T> = std::result::Result<T, InstallError>;
```

## Error Categories

### 1. Path Errors

#### ProjectNotFound
- **When**: Project directory doesn't exist
- **Example**: `Error: Project path does not exist: /home/user/projects/missing`
- **User Action**: Check path spelling, verify directory exists

#### CargoTomlNotFound
- **When**: Cargo.toml not found in project
- **Example**: `Error: Cargo.toml not found in project: /home/user/projects/my-tool`
- **User Action**: Verify it's a Cargo project, check for Cargo.toml

#### BinaryNotFound
- **When**: Compiled binary doesn't exist in target/
- **Example**:
  ```
  Error: Source binary not found: /home/user/projects/my-tool/target/release/my-tool
  Hint: Run 'cargo build --release' in the project directory
  ```
- **User Action**: Run `cargo build --release` or `cargo build`

### 2. Parsing Errors

#### CargoTomlParse
- **When**: Cargo.toml has invalid TOML syntax
- **Example**: `Error: Could not parse Cargo.toml: missing field 'name'`
- **User Action**: Fix Cargo.toml syntax

#### BinaryNameNotFound
- **When**: Cargo.toml missing `[package].name`
- **Example**: `Error: Binary name not found in Cargo.toml`
- **User Action**: Add name field to `[package]` section

### 3. Installation Errors

#### BinaryAlreadyExists
- **When**: Binary already installed at destination
- **Example**: `Error: Binary already exists in installation directory: ~/.local/softwarewrighter/bin/my-tool`
- **User Action**: Uninstall first or use different name with --rename

#### BinaryNotInstalled
- **When**: Trying to uninstall non-existent binary
- **Example**: `Error: Binary not installed: my-tool`
- **User Action**: Check installed binaries with --list

### 4. System Errors

#### Io
- **When**: File system operations fail
- **Examples**:
  - `Error: IO error: Permission denied (os error 13)`
  - `Error: IO error: No space left on device (os error 28)`
- **User Action**: Check permissions, disk space, file locks

#### HomeNotFound
- **When**: HOME environment variable not set
- **Example**: `Error: Home directory not found`
- **User Action**: Set HOME environment variable

### 5. Validation Errors

#### InvalidBinaryName
- **When**: Binary name contains invalid characters
- **Example**: `Error: Invalid binary name: my/tool`
- **User Action**: Use valid filename characters

#### NoOperationSpecified
- **When**: No operation flag provided
- **Example**: `Error: No operation specified. Use --project, --uninstall, --list, or --setup-install-dir`
- **User Action**: Specify one of the required operations

#### InvalidSortOrder
- **When**: Invalid --sort option provided
- **Example**: `Error: Invalid sort order: random. Valid options: name, newest, oldest`
- **User Action**: Use valid sort order

## Error Flow

```
┌─────────────────────────────────────┐
│     Operation (e.g., Validator)     │
│                                     │
│  fs::metadata(path)?                │
│     ↓ (if path not found)           │
│  io::Error                          │
└───────────────┬─────────────────────┘
                │
                v
┌─────────────────────────────────────┐
│     Wrap in InstallError            │
│                                     │
│  Err(InstallError::ProjectNotFound( │
│      path.to_path_buf()             │
│  ))                                 │
└───────────────┬─────────────────────┘
                │
                │ Propagate with ?
                v
┌─────────────────────────────────────┐
│          Caller Function            │
│                                     │
│  let result = validate()?;          │
│  // Error bubbles up                │
└───────────────┬─────────────────────┘
                │
                │ Propagate with ?
                v
┌─────────────────────────────────────┐
│          main() Function            │
│                                     │
│  match run() {                      │
│    Ok(_) => exit(0),                │
│    Err(e) => {                      │
│      eprintln!("Error: {}", e);     │
│      exit(1);                       │
│    }                                │
│  }                                  │
└─────────────────────────────────────┘
```

## Using thiserror

The `thiserror` crate provides:

1. **Automatic Display implementation**: From `#[error("...")]` attribute
2. **From trait implementation**: For `#[from]` attributes
3. **Error trait implementation**: Automatic error trait impl
4. **Source chain**: Tracks error causes

### Example: Automatic From Conversion

```rust
#[error("IO error: {0}")]
Io(#[from] std::io::Error),
```

This allows:
```rust
// Automatic conversion with ?
let contents = fs::read_to_string(path)?;
// io::Error automatically converts to InstallError::Io
```

## Error Message Design

### Principles

1. **Be Specific**: Include paths, names, details
2. **Be Actionable**: Suggest next steps
3. **Be Concise**: Short, clear messages
4. **Be Consistent**: Same format across errors

### Examples

#### Good Error Message
```
Error: Source binary not found: /home/user/projects/my-tool/target/release/my-tool
Hint: Run 'cargo build --release' in the project directory
```

**Why good:**
- Shows exact path
- Suggests specific action
- Clear what went wrong

#### Poor Error Message (Don't do this)
```
Error: Binary not found
```

**Why poor:**
- No path information
- No suggestion
- Vague

## Testing Error Cases

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_not_found_error() {
        let path = PathBuf::from("/nonexistent");
        let error = InstallError::ProjectNotFound(path.clone());

        let message = format!("{}", error);
        assert!(message.contains("/nonexistent"));
        assert!(message.contains("does not exist"));
    }

    #[test]
    fn test_binary_not_found_includes_hint() {
        let path = PathBuf::from("/path/to/binary");
        let error = InstallError::BinaryNotFound(path);

        let message = format!("{}", error);
        assert!(message.contains("cargo build"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "permission denied"
        );

        let install_error: InstallError = io_error.into();
        assert!(matches!(install_error, InstallError::Io(_)));
    }
}
```

### Integration Tests

```rust
#[test]
fn test_missing_project_error() {
    let result = Command::new("sw-install")
        .args(&["-p", "/nonexistent"])
        .output()
        .expect("Failed to execute");

    assert!(!result.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("Project path does not exist"));
}
```

## Error Propagation Pattern

### Using ? Operator

```rust
fn validate(&self) -> Result<ValidationResult> {
    // Automatically propagate errors
    self.validate_project_path()?;
    self.validate_cargo_toml()?;
    let name = self.extract_binary_name()?;
    self.validate_source_binary(&name)?;

    Ok(ValidationResult { binary_name: name })
}
```

Each `?` operator:
1. If `Ok(value)`: Unwraps and continues
2. If `Err(e)`: Returns error immediately

### Manual Error Handling

```rust
fn validate_project_path(&self) -> Result<()> {
    if !self.config.project_path.exists() {
        return Err(InstallError::ProjectNotFound(
            self.config.project_path.clone()
        ));
    }
    Ok(())
}
```

## Exit Codes

The application uses standard exit codes:

- **0**: Success
- **1**: Error (any InstallError)

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

## Design Decisions

### Why thiserror Over anyhow?

**thiserror:**
- Library error type
- Specific error variants
- Type-safe error handling
- Good for reusable code

**anyhow:**
- Application error type
- Generic error handling
- Context chaining
- Good for quick prototyping

**Choice**: thiserror provides better API for users of the library

### Why enum Over struct?

**Enum advantages:**
- Explicit error cases
- Pattern matching
- Type safety
- Exhaustive checking

### Why Include Paths in Errors?

**Benefits:**
1. **Debugging**: Know exactly which file failed
2. **User clarity**: Understand what to fix
3. **Reproducibility**: Can recreate issue
4. **Logging**: Better error logs

## See Also

- [CLI Interface](CLI-Interface) - Top-level error handling
- [Validator](Validator) - Validation errors
- [Installer](Installer) - Installation errors
- [Testing Strategy](Testing-Strategy) - Testing error cases
