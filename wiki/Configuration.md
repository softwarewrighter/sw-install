# Configuration

The Configuration module (`config.rs`) centralizes configuration and path resolution logic.

## Responsibilities

- Store installation parameters
- Compute source binary paths
- Compute destination paths
- Handle rename logic
- Path expansion (e.g., `~` to home directory)

## Data Structure

```rust
pub struct InstallConfig {
    pub project_path: PathBuf,
    pub binary_name: Option<String>,
    pub use_debug: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub test_dir: Option<PathBuf>,
}
```

### Fields

- **project_path**: Path to the Cargo project being installed
- **binary_name**: Optional rename for the binary (if different from Cargo.toml name)
- **use_debug**: Whether to install debug build (`true`) or release (`false`)
- **verbose**: Enable verbose output
- **dry_run**: Preview mode without actual file operations
- **test_dir**: Override destination directory (for testing)

## Key Methods

### Constructor

```rust
impl InstallConfig {
    pub fn new(
        project_path: PathBuf,
        binary_name: Option<String>,
        use_debug: bool,
        verbose: bool,
        dry_run: bool,
        test_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            project_path,
            binary_name,
            use_debug,
            verbose,
            dry_run,
            test_dir,
        }
    }
}
```

### Destination Directory

Computes the installation directory path:

```rust
pub fn destination_dir(&self) -> Result<PathBuf> {
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

**Returns**: `~/.local/softwarewrighter/bin/` or test directory

### Source Binary Path

Computes the path to the compiled binary:

```rust
pub fn source_binary_path(&self, actual_name: &str) -> PathBuf {
    self.project_path
        .join("target")
        .join(self.target_subdir())
        .join(actual_name)
}
```

**Returns**: `<project>/target/[release|debug]/<name>`

### Destination Binary Path

Computes the final installation path:

```rust
pub fn destination_binary_path(&self, actual_name: &str) -> Result<PathBuf> {
    let dest_dir = self.destination_dir()?;
    let filename = self.binary_name
        .as_deref()
        .unwrap_or(actual_name);

    Ok(dest_dir.join(filename))
}
```

**Returns**: `<dest_dir>/<rename_or_actual_name>`

### Target Subdirectory

Returns the appropriate target subdirectory:

```rust
pub fn target_subdir(&self) -> &str {
    if self.use_debug {
        "debug"
    } else {
        "release"
    }
}
```

## Path Resolution Examples

### Example 1: Basic Release Installation

**Input:**
```rust
InstallConfig {
    project_path: PathBuf::from("/home/user/projects/my-tool"),
    binary_name: None,
    use_debug: false,
    verbose: false,
    dry_run: false,
    test_dir: None,
}
```

**Computed Paths:**
- `destination_dir()` → `/home/user/.local/softwarewrighter/bin/`
- `source_binary_path("my-tool")` → `/home/user/projects/my-tool/target/release/my-tool`
- `destination_binary_path("my-tool")` → `/home/user/.local/softwarewrighter/bin/my-tool`
- `target_subdir()` → `"release"`

### Example 2: Debug Build with Rename

**Input:**
```rust
InstallConfig {
    project_path: PathBuf::from("/home/user/projects/ask"),
    binary_name: Some(String::from("ask-dev")),
    use_debug: true,
    verbose: true,
    dry_run: false,
    test_dir: None,
}
```

**Computed Paths:**
- `destination_dir()` → `/home/user/.local/softwarewrighter/bin/`
- `source_binary_path("ask")` → `/home/user/projects/ask/target/debug/ask`
- `destination_binary_path("ask")` → `/home/user/.local/softwarewrighter/bin/ask-dev`
- `target_subdir()` → `"debug"`

### Example 3: Test Mode

**Input:**
```rust
InstallConfig {
    project_path: PathBuf::from("/home/user/projects/my-tool"),
    binary_name: None,
    use_debug: false,
    verbose: false,
    dry_run: false,
    test_dir: Some(PathBuf::from("/tmp/test-bin")),
}
```

**Computed Paths:**
- `destination_dir()` → `/tmp/test-bin/`
- `source_binary_path("my-tool")` → `/home/user/projects/my-tool/target/release/my-tool`
- `destination_binary_path("my-tool")` → `/tmp/test-bin/my-tool`
- `target_subdir()` → `"release"`

## Path Transformation Flow

```
User Input                 Config Field            Computed Path
──────────                 ────────────            ─────────────

~/projects/ask        →    project_path       →    /home/user/projects/ask
                           (expanded)

--rename ask-dev      →    binary_name        →    Used in destination path
                           = Some("ask-dev")

--type debug          →    use_debug = true   →    target/debug/
(default: release)         use_debug = false       target/release/

(none)                →    test_dir = None    →    ~/.local/softwarewrighter/bin/
--test-dir /tmp/test  →    test_dir = Some()  →    /tmp/test/
```

## Integration with Other Components

### Used By

1. **Validator**: Accesses paths to validate project structure
2. **Installer**: Uses paths to copy binary and set permissions
3. **CLI**: Builds config from command-line arguments

### Dependencies

- **std::path::PathBuf**: Platform-independent path handling
- **std::env**: Access to HOME environment variable
- **error::InstallError**: Error handling

## Error Cases

The configuration module can produce the following errors:

1. **HomeNotFound**: `HOME` environment variable not set
   ```rust
   Err(InstallError::HomeNotFound)
   ```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_dir() {
        let config = InstallConfig::new(
            PathBuf::from("/test"),
            None,
            false,
            false,
            false,
            None,
        );

        let dest = config.destination_dir().unwrap();
        assert!(dest.ends_with(".local/softwarewrighter/bin"));
    }

    #[test]
    fn test_source_binary_path_release() {
        let config = InstallConfig::new(
            PathBuf::from("/projects/ask"),
            None,
            false,  // use_debug = false
            false,
            false,
            None,
        );

        let path = config.source_binary_path("ask");
        assert_eq!(path, PathBuf::from("/projects/ask/target/release/ask"));
    }

    #[test]
    fn test_source_binary_path_debug() {
        let config = InstallConfig::new(
            PathBuf::from("/projects/ask"),
            None,
            true,  // use_debug = true
            false,
            false,
            None,
        );

        let path = config.source_binary_path("ask");
        assert_eq!(path, PathBuf::from("/projects/ask/target/debug/ask"));
    }

    #[test]
    fn test_destination_with_rename() {
        let config = InstallConfig::new(
            PathBuf::from("/projects/ask"),
            Some("ask-dev".to_string()),
            false,
            false,
            false,
            None,
        );

        let path = config.destination_binary_path("ask").unwrap();
        assert!(path.ends_with("ask-dev"));
    }

    #[test]
    fn test_test_dir_override() {
        let config = InstallConfig::new(
            PathBuf::from("/projects/ask"),
            None,
            false,
            false,
            false,
            Some(PathBuf::from("/tmp/test")),
        );

        let dest = config.destination_dir().unwrap();
        assert_eq!(dest, PathBuf::from("/tmp/test"));
    }
}
```

## Design Decisions

### Why Centralize Path Logic?

1. **Single Source of Truth**: All path computations in one place
2. **Testability**: Easy to test path logic without file I/O
3. **Consistency**: Ensures all components use the same paths
4. **Maintainability**: Changes to path structure require updates in one place

### Why PathBuf Over String?

1. **Platform Independence**: Works on Windows, macOS, Linux
2. **Type Safety**: Compiler ensures correct path operations
3. **Convenience**: Methods like `join()`, `parent()`, `ends_with()`

### Why Option for binary_name?

Allows rename to be optional:
- `None` → Use original name from Cargo.toml
- `Some(name)` → Rename to specified name

## See Also

- [CLI Interface](CLI-Interface) - How config is built from arguments
- [Validator](Validator) - Uses config to validate paths
- [Installer](Installer) - Uses config to perform installation
- [Error Handling](Error-Handling) - Error types returned
