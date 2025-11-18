# Installer

The Installer module (`installer.rs`) performs the actual binary installation operations.

## Responsibilities

- Create destination directory if it doesn't exist
- Copy binary from source to destination
- Set executable permissions on Unix systems
- Report success or failure

## Data Structure

```rust
pub struct Installer<'a> {
    config: &'a InstallConfig,
    binary_name: String,
    output: Box<dyn OutputHandler>,
}
```

## Key Methods

### Constructor

```rust
impl<'a> Installer<'a> {
    pub fn new(
        config: &'a InstallConfig,
        binary_name: String,
        output: Box<dyn OutputHandler>,
    ) -> Self {
        Self {
            config,
            binary_name,
            output,
        }
    }
}
```

### Main Installation

```rust
pub fn install(&self) -> Result<()> {
    let dest_dir = self.create_destination_dir()?;
    let dest_path = self.copy_binary(&dest_dir)?;
    self.set_permissions(&dest_path)?;

    let display_name = self.config.binary_name
        .as_deref()
        .unwrap_or(&self.binary_name);

    self.output.success(&format!(
        "Successfully installed: {}",
        display_name
    ));
    self.output.info(&format!(
        "Location: {}",
        dest_path.display()
    ));

    Ok(())
}
```

## Installation Steps

### 1. Create Destination Directory

```rust
fn create_destination_dir(&self) -> Result<PathBuf> {
    self.output.step("Creating destination directory");

    let dest_dir = self.config.destination_dir()?;

    if !self.config.dry_run {
        fs::create_dir_all(&dest_dir)?;
    }

    Ok(dest_dir)
}
```

**Operation:**
- Get destination directory path from config
- Create all parent directories if needed
- Skip if in dry-run mode

**Returns:** Destination directory PathBuf

### 2. Copy Binary

```rust
fn copy_binary(&self, dest_dir: &Path) -> Result<PathBuf> {
    self.output.step("Copying binary");

    let source = self.config.source_binary_path(&self.binary_name);
    let dest = self.config.destination_binary_path(&self.binary_name)?;

    if !self.config.dry_run {
        fs::copy(&source, &dest)?;
    }

    self.output.info(&format!("From: {}", source.display()));
    self.output.info(&format!("To:   {}", dest.display()));

    Ok(dest)
}
```

**Operation:**
- Compute source path (target/release or target/debug)
- Compute destination path (with rename if specified)
- Copy file
- Skip if in dry-run mode

**Returns:** Destination binary PathBuf

### 3. Set Permissions

```rust
#[cfg(unix)]
fn set_permissions(&self, binary_path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    self.output.step("Setting executable permissions");

    if !self.config.dry_run {
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(binary_path, perms)?;
    }

    Ok(())
}

#[cfg(not(unix))]
fn set_permissions(&self, _binary_path: &Path) -> Result<()> {
    // No-op on non-Unix platforms
    Ok(())
}
```

**Operation:**
- Set permissions to 0o755 (rwxr-xr-x)
- Owner: read, write, execute
- Group: read, execute
- Others: read, execute
- Skip if in dry-run mode
- Unix-only (no-op on Windows)

## Installation Flow

```
┌─────────────────────────────────────────┐
│        Installer::install()             │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   create_destination_dir()              │
│   • Get dest path from config           │
│   • fs::create_dir_all()                │
│   → ~/.local/softwarewrighter/bin/      │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   copy_binary()                         │
│   • Get source path from config         │
│   • Get dest path from config           │
│   • fs::copy(source, dest)              │
│   → Binary copied                       │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   set_permissions()                     │
│   • fs::set_permissions(0o755)          │
│   → Binary executable                   │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Report Success                        │
│   • Display installed binary name       │
│   • Display installation location       │
└─────────────────────────────────────────┘
```

## Output Examples

### Normal Mode

```
Successfully installed: my-tool
Location: /home/user/.local/softwarewrighter/bin/my-tool
```

### Verbose Mode

```
[5/7] Creating destination directory ... OK
[6/7] Copying binary ... OK
From: /home/user/projects/my-tool/target/release/my-tool
To:   /home/user/.local/softwarewrighter/bin/my-tool
[7/7] Setting executable permissions ... OK

Successfully installed: my-tool
Location: /home/user/.local/softwarewrighter/bin/my-tool
```

### Dry-Run Mode

```
Would: Create destination directory: ~/.local/softwarewrighter/bin
Would: Copy binary from /home/user/projects/my-tool/target/release/my-tool
Would: Copy binary to ~/.local/softwarewrighter/bin/my-tool
Would: Set executable permissions on ~/.local/softwarewrighter/bin/my-tool
Dry-run complete: No changes made
```

## Error Handling

### Common Errors

1. **IO Error during directory creation**
   ```
   Error: IO error: Permission denied (os error 13)
   ```

2. **IO Error during copy**
   ```
   Error: IO error: No space left on device (os error 28)
   ```

3. **IO Error during permission setting**
   ```
   Error: IO error: Operation not permitted (os error 1)
   ```

All IO errors are wrapped in `InstallError::Io(std::io::Error)` and propagate up.

## Platform Considerations

### Unix (Linux, macOS)

- Uses `fs::Permissions::from_mode(0o755)`
- Sets executable bit for owner, group, others
- Requires write permission to destination directory

### Windows

- `set_permissions()` is a no-op
- Windows handles executability differently (.exe extension)
- Currently not a target platform

## Integration with Other Components

```
Validator → ValidationResult → Installer
               (binary_name)
```

**Usage:**
```rust
let result = validator.validate()?;
let installer = Installer::new(&config, result.binary_name, output);
installer.install()?;
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_install_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let test_dest = temp_dir.path().join("test-dest");

        let config = InstallConfig::new(
            PathBuf::from("/fake"),
            None, false, false, false,
            Some(test_dest.clone()),
        );

        // ... test that directory is created
    }

    #[test]
    fn test_install_copies_binary() {
        // Create source binary
        let source_dir = tempdir().unwrap();
        let binary_path = source_dir.path().join("test-binary");
        fs::write(&binary_path, "test content").unwrap();

        // Install and verify copy
        // ...
    }

    #[test]
    fn test_install_dry_run_no_changes() {
        let config = InstallConfig::new(
            PathBuf::from("/fake"),
            None, false, false,
            true,  // dry_run = true
            None,
        );

        // Verify no files are created in dry-run mode
        // ...
    }

    #[test]
    fn test_install_with_rename() {
        let config = InstallConfig::new(
            PathBuf::from("/fake"),
            Some("renamed-binary".to_string()),
            false, false, false, None,
        );

        // Verify binary is installed with new name
        // ...
    }
}
```

### Integration Tests

```rust
#[test]
fn test_full_install_workflow() {
    // 1. Create temporary project with Cargo.toml
    // 2. Build fake binary
    // 3. Run validator
    // 4. Run installer
    // 5. Verify binary exists and is executable
    // 6. Clean up
}
```

## Design Decisions

### Why Three Separate Methods?

1. **Single Responsibility**: Each method does one thing
2. **Testability**: Easy to test each step independently
3. **Error Isolation**: Clear where failures occur
4. **Dry-Run Support**: Can skip operations conditionally

### Why Not Use a Builder Pattern?

- Installation is a simple three-step process
- Builder would add unnecessary complexity
- Linear flow is clear and easy to understand

### Why Accept binary_name in Constructor?

- Ensures installer always has validated data
- Type-safe: can't create installer without validation
- Clear contract: validation must happen first

## See Also

- [Validator](Validator) - Validates before installation
- [Configuration](Configuration) - Provides paths
- [Output Handler](Output-Handler) - Output formatting
- [Error Handling](Error-Handling) - Error types
