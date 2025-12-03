# Validator

The Validator module (`validator.rs`) performs pre-flight checks before installation operations.

## Responsibilities

- Verify project path exists and is a directory
- Detect project structure type (simple, workspace, multi-component)
- Parse Cargo.toml to extract binary name
- Verify compiled binary exists in target directory
- Return validation results including source binary path for use by installer

## Project Types

The validator supports three distinct project structures:

### Simple Project
Single Cargo.toml with `[package]` section:
```
my-project/
  Cargo.toml      # [package] name = "my-app"
  src/main.rs
  target/release/my-app
```

### Workspace Project
Cargo.toml with `[workspace]` section containing member crates:
```
my-workspace/
  Cargo.toml      # [workspace] members = ["crates/*"]
  crates/
    my-cli/
      Cargo.toml  # [package] with [[bin]] or src/main.rs
      src/main.rs
  target/release/my-cli
```

### Multi-Component Project
No root Cargo.toml, with workspace Cargo.toml files in `components/`:
```
my-repo/
  components/
    cli-component/
      Cargo.toml    # [workspace] members = ["crates/cli"]
      crates/
        cli/
          Cargo.toml
          src/main.rs
      target/release/my-cli
    lib-component/
      Cargo.toml    # Another workspace (library only)
```

## Data Structure

```rust
pub struct Validator<'a> {
    config: &'a InstallConfig,
    output: &'a dyn OutputHandler,
}

pub struct ValidationResult {
    pub binary_name: String,
    pub source_binary_path: PathBuf,
}

enum ProjectType {
    Simple,
    Workspace,
    MultiComponent { component_path: PathBuf },
}
```

## Key Methods

### Constructor

```rust
impl<'a> Validator<'a> {
    pub fn new(config: &'a InstallConfig, output: Box<dyn OutputHandler>) -> Self {
        Self { config, output }
    }
}
```

### Main Validation

```rust
pub fn validate(&self) -> Result<ValidationResult> {
    self.validate_project_path()?;
    let project_type = self.detect_project_type()?;
    let binary_name = self.extract_binary_name_for_type(&project_type)?;
    let source_binary_path = self.validate_source_binary_for_type(&binary_name, &project_type)?;

    Ok(ValidationResult { binary_name, source_binary_path })
}
```

## Validation Steps

### 1. Validate Project Path

Ensures the project directory exists:

```rust
fn validate_project_path(&self) -> Result<()> {
    self.output.step("Checking project path");

    if !self.config.project_path.exists() {
        return Err(InstallError::ProjectNotFound(
            self.config.project_path.clone()
        ));
    }

    if !self.config.project_path.is_dir() {
        return Err(InstallError::ProjectNotFound(
            self.config.project_path.clone()
        ));
    }

    Ok(())
}
```

**Checks:**
- Path exists
- Path is a directory (not a file)

**Error:** `InstallError::ProjectNotFound(PathBuf)`

### 2. Detect Project Type

Identifies the project structure:

```rust
fn detect_project_type(&self) -> Result<ProjectType> {
    let cargo_toml_path = self.config.project_path.join("Cargo.toml");

    if cargo_toml_path.exists() {
        // Parse and check for [workspace] or [package]
        if has_workspace_section { return Ok(ProjectType::Workspace); }
        if has_package_section { return Ok(ProjectType::Simple); }
    }

    // Check for components/ directory with workspace Cargo.toml files
    let components_dir = self.config.project_path.join("components");
    if components_dir.is_dir() {
        if let Some(component_path) = self.find_component_with_binary(&components_dir) {
            return Ok(ProjectType::MultiComponent { component_path });
        }
    }

    Err(InstallError::CargoTomlNotFound(self.config.project_path.clone()))
}
```

**Detection Order:**
1. Check for root Cargo.toml with `[workspace]` -> Workspace
2. Check for root Cargo.toml with `[package]` -> Simple
3. Check for `components/` directory with binary crates -> MultiComponent

**Error:** `InstallError::CargoTomlNotFound(PathBuf)`

### 3. Extract Binary Name

Parses Cargo.toml to get the binary name:

```rust
fn extract_binary_name(&self) -> Result<String> {
    self.output.step("Parsing Cargo.toml");

    let cargo_toml_path = self.config.project_path.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| InstallError::Io(e))?;

    let toml: toml::Value = toml::from_str(&contents)
        .map_err(|e| InstallError::CargoTomlParse(e.to_string()))?;

    let name = toml
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or(InstallError::BinaryNameNotFound)?;

    self.output.info(&format!("Binary name: {}", name));

    Ok(name.to_string())
}
```

**Process:**
1. Read Cargo.toml file
2. Parse as TOML
3. Extract `[package].name` field
4. Return name as String

**Errors:**
- `InstallError::Io`: File read error
- `InstallError::CargoTomlParse`: TOML parsing error
- `InstallError::BinaryNameNotFound`: Missing `[package].name`

### 4. Validate Source Binary

Verifies the compiled binary exists:

```rust
fn validate_source_binary(&self, binary_name: &str) -> Result<()> {
    self.output.step("Checking source binary");

    let source_path = self.config.source_binary_path(binary_name);

    if !source_path.exists() {
        return Err(InstallError::BinaryNotFound(source_path));
    }

    Ok(())
}
```

**Checks:**
- Binary exists at `<project>/target/[release|debug]/<name>`

**Error:** `InstallError::BinaryNotFound(PathBuf)`

## Validation Flow Diagram

```
┌─────────────────────────────────────────┐
│        Validator::validate()            │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Step 1: validate_project_path()       │
│   - Path exists?                        │
│   - Is directory?                       │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Step 2: detect_project_type()         │
│   - Check root Cargo.toml               │
│   - [workspace] -> Workspace            │
│   - [package] -> Simple                 │
│   - No root -> check components/        │
│   -> Returns: ProjectType               │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Step 3: extract_binary_name_for_type()│
│   - Simple: [package].name              │
│   - Workspace: scan members for binaries│
│   - MultiComponent: scan component      │
│   -> Returns: "binary-name"             │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│Step 4: validate_source_binary_for_type()│
│   - Simple/Workspace: project/target/   │
│   - MultiComponent: component/target/   │
│   -> Returns: PathBuf to binary         │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Return: ValidationResult              │
│   { binary_name, source_binary_path }   │
└─────────────────────────────────────────┘
```

## Example Cargo.toml Parsing

### Valid Cargo.toml

```toml
[package]
name = "my-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = "4.0"
```

**Extracted name:** `"my-tool"`

### Invalid Cargo.toml (Missing name)

```toml
[package]
version = "0.1.0"
edition = "2021"
```

**Error:** `InstallError::BinaryNameNotFound`

## Integration with Other Components

```
CLI -> Config -> Validator -> Installer
                    |
             ValidationResult
             { binary_name, source_binary_path }
```

**Usage:**
```rust
let validator = Validator::new(&config, output);
let result = validator.validate()?;
let installer = Installer::new(&config, result.binary_name, result.source_binary_path, output);
```

## Error Messages

### Project Not Found

```
Error: Project path does not exist: /home/user/projects/missing
```

### Cargo.toml Not Found

```
Error: Cargo.toml not found in project: /home/user/projects/my-tool
```

### Binary Not Found

```
Error: Source binary not found: /home/user/projects/my-tool/target/release/my-tool
```

**Suggestion**: Run `cargo build --release` in the project directory

### Binary Name Not Found

```
Error: Binary name not found in Cargo.toml
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_validate_missing_project() {
        let config = InstallConfig::new(
            PathBuf::from("/nonexistent"),
            None, false, false, false, None,
        );
        let output = Box::new(TestOutput::new());
        let validator = Validator::new(&config, output);

        let result = validator.validate();
        assert!(matches!(result, Err(InstallError::ProjectNotFound(_))));
    }

    #[test]
    fn test_validate_missing_cargo_toml() {
        let temp_dir = tempdir().unwrap();
        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None, false, false, false, None,
        );
        let output = Box::new(TestOutput::new());
        let validator = Validator::new(&config, output);

        let result = validator.validate();
        assert!(matches!(result, Err(InstallError::CargoTomlNotFound(_))));
    }

    #[test]
    fn test_validate_missing_binary() {
        let temp_dir = tempdir().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"\n").unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None, false, false, false, None,
        );
        let output = Box::new(TestOutput::new());
        let validator = Validator::new(&config, output);

        let result = validator.validate();
        assert!(matches!(result, Err(InstallError::BinaryNotFound(_))));
    }

    #[test]
    fn test_validate_success() {
        // Create complete test project
        let temp_dir = tempdir().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"\n").unwrap();

        let target_dir = temp_dir.path().join("target/release");
        fs::create_dir_all(&target_dir).unwrap();
        let binary = target_dir.join("test");
        fs::write(&binary, "fake binary").unwrap();

        let config = InstallConfig::new(
            temp_dir.path().to_path_buf(),
            None, false, false, false, None,
        );
        let output = Box::new(TestOutput::new());
        let validator = Validator::new(&config, output);

        let result = validator.validate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().binary_name, "test");
    }
}
```

## Design Decisions

### Why Separate Validation?

1. **Fail Fast**: Detect issues before file operations
2. **Clear Error Messages**: Specific validation failures
3. **Testability**: Easy to test validation logic in isolation
4. **Safety**: No partial installations due to late failures

### Why Return ValidationResult?

- Encapsulates validated data (binary name)
- Type-safe way to pass validated info to installer
- Clear contract: installer receives only validated data

### Why Use OutputHandler?

- Consistent output across normal/verbose/dry-run modes
- Separation of concerns: validation logic vs. display logic
- Easy to test without console output

## See Also

- [Configuration](Configuration) - Config used for path resolution
- [Installer](Installer) - Uses validation results
- [Output Handler](Output-Handler) - Output abstraction
- [Error Handling](Error-Handling) - Error types
