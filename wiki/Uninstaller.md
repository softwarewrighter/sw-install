# Uninstaller

The Uninstaller module (`uninstaller.rs`) handles the removal of installed binaries.

## Responsibilities

- Validate that binary exists in installation directory
- Remove binary file from installation directory
- Report success or failure

## Data Structure

```rust
pub struct Uninstaller {
    binary_name: String,
    verbose: bool,
    dry_run: bool,
    test_dir: Option<PathBuf>,
    output: Box<dyn OutputHandler>,
}
```

## Key Methods

### Constructor

```rust
impl Uninstaller {
    pub fn new(
        binary_name: String,
        verbose: bool,
        dry_run: bool,
        test_dir: Option<PathBuf>,
        output: Box<dyn OutputHandler>,
    ) -> Self {
        Self {
            binary_name,
            verbose,
            dry_run,
            test_dir,
            output,
        }
    }
}
```

### Main Uninstallation

```rust
pub fn uninstall(&self) -> Result<()> {
    let binary_path = self.binary_path()?;
    self.validate_binary_exists(&binary_path)?;
    self.remove_binary(&binary_path)?;

    self.output.success(&format!(
        "Successfully uninstalled: {}",
        self.binary_name
    ));

    Ok(())
}
```

## Uninstallation Steps

### 1. Compute Binary Path

```rust
fn binary_path(&self) -> Result<PathBuf> {
    let dest_dir = self.destination_dir()?;
    Ok(dest_dir.join(&self.binary_name))
}

fn destination_dir(&self) -> Result<PathBuf> {
    if let Some(test_dir) = &self.test_dir {
        return Ok(test_dir.clone());
    }

    let home = std::env::var("HOME")
        .map_err(|_| InstallError::HomeNotFound)?;

    Ok(PathBuf::from(home)
        .join(".local")
        .join("softwarewrighter")
        .join("bin"))
}
```

**Returns**: Full path to the binary to uninstall

### 2. Validate Binary Exists

```rust
fn validate_binary_exists(&self, binary_path: &Path) -> Result<()> {
    self.output.step("Checking if binary is installed");

    if !binary_path.exists() {
        return Err(InstallError::BinaryNotInstalled(
            self.binary_name.clone()
        ));
    }

    Ok(())
}
```

**Checks:**
- Binary exists at installation path
- Prevents attempting to remove non-existent file

**Error:** `InstallError::BinaryNotInstalled(String)`

### 3. Remove Binary

```rust
fn remove_binary(&self, binary_path: &Path) -> Result<()> {
    self.output.step(&format!(
        "Removing binary: {}",
        binary_path.display()
    ));

    if !self.dry_run {
        fs::remove_file(binary_path)?;
    }

    Ok(())
}
```

**Operation:**
- Delete the binary file
- Skip if in dry-run mode

## Uninstallation Flow

```
┌─────────────────────────────────────────┐
│      Uninstaller::uninstall()           │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   binary_path()                         │
│   • Get destination directory           │
│   • Join with binary name               │
│   → Full path to binary                 │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   validate_binary_exists()              │
│   • Check if file exists                │
│   • Error if not found                  │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   remove_binary()                       │
│   • fs::remove_file(path)               │
│   → Binary removed                      │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Report Success                        │
│   • Display uninstalled binary name     │
└─────────────────────────────────────────┘
```

## Output Examples

### Normal Mode

```
Successfully uninstalled: my-tool
```

### Verbose Mode

```
[1/2] Checking if binary is installed ... OK
[2/2] Removing binary: /home/user/.local/softwarewrighter/bin/my-tool ... OK

Successfully uninstalled: my-tool
```

### Dry-Run Mode

```
Would: Check if binary is installed
Would: Remove binary: ~/.local/softwarewrighter/bin/my-tool
Dry-run complete: No changes made
```

## Usage Examples

### Basic Uninstall

```bash
sw-install --uninstall my-tool
```

### Verbose Uninstall

```bash
sw-install --uninstall my-tool --verbose
```

### Dry-Run Uninstall

```bash
sw-install --uninstall my-tool --dry-run --verbose
```

## Error Handling

### Binary Not Installed

```
Error: Binary not installed: my-tool
```

**User Action**: Check installed binaries with `sw-install --list`

### Permission Denied

```
Error: IO error: Permission denied (os error 13)
```

**User Action**: Check file permissions, may need to fix installation directory permissions

## Integration with CLI

```rust
fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(binary_name) = args.uninstall {
        let output = create_output_handler(args.verbose, args.dry_run);
        let uninstaller = Uninstaller::new(
            binary_name,
            args.verbose,
            args.dry_run,
            args.test_dir,
            output,
        );

        uninstaller.uninstall()?;
    }

    Ok(())
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_uninstall_removes_binary() {
        let temp_dir = tempdir().unwrap();
        let binary = temp_dir.path().join("test-binary");
        fs::write(&binary, "test").unwrap();

        let uninstaller = Uninstaller::new(
            "test-binary".to_string(),
            false, false,
            Some(temp_dir.path().to_path_buf()),
            Box::new(TestOutput::new()),
        );

        assert!(binary.exists());
        uninstaller.uninstall().unwrap();
        assert!(!binary.exists());
    }

    #[test]
    fn test_uninstall_nonexistent_binary() {
        let temp_dir = tempdir().unwrap();

        let uninstaller = Uninstaller::new(
            "nonexistent".to_string(),
            false, false,
            Some(temp_dir.path().to_path_buf()),
            Box::new(TestOutput::new()),
        );

        let result = uninstaller.uninstall();
        assert!(matches!(
            result,
            Err(InstallError::BinaryNotInstalled(_))
        ));
    }

    #[test]
    fn test_uninstall_dry_run_preserves_binary() {
        let temp_dir = tempdir().unwrap();
        let binary = temp_dir.path().join("test-binary");
        fs::write(&binary, "test").unwrap();

        let uninstaller = Uninstaller::new(
            "test-binary".to_string(),
            false,
            true, // dry_run
            Some(temp_dir.path().to_path_buf()),
            Box::new(TestOutput::new()),
        );

        uninstaller.uninstall().unwrap();
        assert!(binary.exists()); // Still exists in dry-run
    }
}
```

## Design Decisions

### Why Separate from Installer?

1. **Single Responsibility**: Each module has one purpose
2. **Simpler Logic**: Uninstall is simpler than install
3. **Independent Testing**: Test each operation separately
4. **Clear API**: Obvious what each module does

### Why Validate Before Removing?

1. **Better Error Messages**: Specific "not installed" error
2. **Prevent Silent Failures**: Know if operation had no effect
3. **User Feedback**: Clear what went wrong

### Why test_dir Option?

- Same as Installer: enables testing without affecting system
- Consistent interface across modules
- Safe testing

## See Also

- [Installer](Installer) - Binary installation
- [CLI Interface](CLI-Interface) - Uninstall command
- [Output Handler](Output-Handler) - Output formatting
- [Error Handling](Error-Handling) - Error types
