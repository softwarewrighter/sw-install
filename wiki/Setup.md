# Setup

The Setup module (`setup.rs`) handles first-time installation directory setup and PATH configuration.

## Responsibilities

- Create installation directory (`~/.local/softwarewrighter/bin/`)
- Detect user's shell (bash or zsh)
- Add PATH configuration to shell config file
- Provide instructions to reload shell

## Key Operations

### 1. Create Installation Directory

```rust
pub fn setup_install_dir(
    verbose: bool,
    dry_run: bool,
    output: Box<dyn OutputHandler>,
) -> Result<()> {
    let dest_dir = get_destination_dir()?;

    output.step(&format!("Creating directory: {}", dest_dir.display()));

    if !dry_run {
        fs::create_dir_all(&dest_dir)?;
    }

    // Continue with shell configuration...
}
```

**Creates**: `~/.local/softwarewrighter/bin/`

### 2. Detect Shell

```rust
fn detect_shell() -> Result<Shell> {
    let shell_path = std::env::var("SHELL")
        .map_err(|_| InstallError::ShellNotDetected)?;

    if shell_path.contains("zsh") {
        Ok(Shell::Zsh)
    } else if shell_path.contains("bash") {
        Ok(Shell::Bash)
    } else {
        Ok(Shell::Other(shell_path))
    }
}

enum Shell {
    Bash,
    Zsh,
    Other(String),
}
```

**Detects** based on `SHELL` environment variable:
- `/bin/zsh` → `Shell::Zsh`
- `/bin/bash` → `Shell::Bash`
- Other → `Shell::Other`

### 3. Get Shell Config File

```rust
fn get_shell_config_file(shell: &Shell) -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| InstallError::HomeNotFound)?;

    let config_file = match shell {
        Shell::Bash => ".bashrc",
        Shell::Zsh => ".zshrc",
        Shell::Other(_) => ".profile",
    };

    Ok(PathBuf::from(home).join(config_file))
}
```

**Returns**:
- Bash → `~/.bashrc`
- Zsh → `~/.zshrc`
- Other → `~/.profile`

### 4. Check if Already Configured

```rust
fn is_path_configured(config_file: &Path) -> Result<bool> {
    if !config_file.exists() {
        return Ok(false);
    }

    let contents = fs::read_to_string(config_file)?;
    Ok(contents.contains("softwarewrighter/bin"))
}
```

**Checks**: Whether config file already mentions installation directory

### 5. Add PATH Configuration

```rust
fn add_path_config(
    config_file: &Path,
    dry_run: bool,
    output: &dyn OutputHandler,
) -> Result<()> {
    let config_text = "\n# Added by sw-install\nexport PATH=\"$HOME/.local/softwarewrighter/bin:$PATH\"\n";

    output.step(&format!(
        "Adding PATH configuration to: {}",
        config_file.display()
    ));

    if !dry_run {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(config_file)?;

        file.write_all(config_text.as_bytes())?;
    }

    Ok(())
}
```

**Appends** to shell config:
```bash
# Added by sw-install
export PATH="$HOME/.local/softwarewrighter/bin:$PATH"
```

### 6. Display Instructions

```rust
fn display_instructions(
    shell: &Shell,
    config_file: &Path,
    output: &dyn OutputHandler,
) {
    output.success("Setup complete!");
    output.info(&format!("Created: ~/.local/softwarewrighter/bin/"));
    output.info(&format!("Updated: {}", config_file.display()));
    output.info("");
    output.info("To activate, run:");
    output.info(&format!("  source {}", config_file.display()));
}
```

## Setup Flow

```
┌─────────────────────────────────────────┐
│      setup_install_dir()                │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Create Installation Directory         │
│   fs::create_dir_all()                  │
│   → ~/.local/softwarewrighter/bin/      │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Detect Shell                          │
│   env::var("SHELL")                     │
│   → Shell::Zsh or Shell::Bash           │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Get Shell Config File                 │
│   → ~/.zshrc or ~/.bashrc               │
└────────────────┬────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Check if Already Configured           │
│   Read config file, search for PATH     │
└────────────────┬────────────────────────┘
                 │
         ┌───────┴────────┐
         │                │
         v                v
    Already          Not configured
    configured           │
         │               v
         │    ┌──────────────────────┐
         │    │  Add PATH Config     │
         │    │  Append to config    │
         │    └──────────┬───────────┘
         │               │
         └───────┬───────┘
                 │
                 v
┌─────────────────────────────────────────┐
│   Display Instructions                  │
│   "Run: source ~/.zshrc"                │
└─────────────────────────────────────────┘
```

## Output Examples

### First Time Setup

```
$ sw-install --setup-install-dir

Creating directory: /home/user/.local/softwarewrighter/bin ... OK
Adding PATH configuration to: /home/user/.zshrc ... OK

Setup complete!
Created: ~/.local/softwarewrighter/bin/
Updated: /home/user/.zshrc

To activate, run:
  source /home/user/.zshrc
```

### Already Configured

```
$ sw-install --setup-install-dir

Directory already exists: /home/user/.local/softwarewrighter/bin
PATH already configured in: /home/user/.zshrc

Setup complete! No changes needed.
```

### Verbose Mode

```
$ sw-install --setup-install-dir --verbose

[1/3] Creating installation directory ... OK
  Directory: /home/user/.local/softwarewrighter/bin
[2/3] Detecting shell ... zsh
  Config file: /home/user/.zshrc
[3/3] Adding PATH configuration ... OK

Setup complete!
Created: ~/.local/softwarewrighter/bin/
Updated: /home/user/.zshrc

To activate, run:
  source /home/user/.zshrc
```

### Dry-Run Mode

```
$ sw-install --setup-install-dir --dry-run

Would: Create directory: ~/.local/softwarewrighter/bin
Would: Detect shell type
Would: Check /home/user/.zshrc
Would: Add PATH configuration to /home/user/.zshrc
Dry-run complete: No changes made
```

## PATH Configuration Format

The setup adds the following to your shell config:

```bash
# Added by sw-install
export PATH="$HOME/.local/softwarewrighter/bin:$PATH"
```

**Why this format:**
- Comment helps identify source
- Prepends to PATH (takes priority)
- Uses `$HOME` (works across users)
- Shell-agnostic (works in bash and zsh)

## Usage

```bash
# First-time setup
sw-install --setup-install-dir

# With verbose output
sw-install --setup-install-dir --verbose

# Preview without making changes
sw-install --setup-install-dir --dry-run
```

## Error Handling

### Shell Not Detected

```
Error: Could not detect shell. Please set SHELL environment variable.
```

### Home Not Found

```
Error: Home directory not found
```

### Permission Denied

```
Error: IO error: Permission denied (os error 13)
```

**User Action**: Check permissions on home directory and shell config file

## Design Decisions

### Why Modify Shell Config?

**Alternatives considered:**
1. Manual PATH setting (user must remember)
2. System-wide installation (/usr/local/bin) - requires sudo
3. Symlinks (complex, brittle)

**Choice**: Modify user shell config
- No sudo required
- Persistent across sessions
- Standard practice
- Easy to understand

### Why Check if Already Configured?

1. **Idempotency**: Can run setup multiple times safely
2. **Avoid Duplicates**: Don't add PATH multiple times
3. **User Experience**: Clear feedback

### Why Append Instead of Modify?

- Safe: Doesn't change existing content
- Simple: No parsing required
- Reversible: Easy to undo manually

## See Also

- [CLI Interface](CLI-Interface) - Setup command
- [Installer](Installer) - Uses installation directory
- [Error Handling](Error-Handling) - Setup errors
- [Testing Strategy](Testing-Strategy) - Testing setup
