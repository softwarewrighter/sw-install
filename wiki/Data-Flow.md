# Data Flow

This page describes how data flows through the sw-install application from user input to final output.

## Overall Data Flow

```
┌──────────────────────────────────────────────────────────────┐
│                      Complete Data Flow                      │
│                                                              │
│  ┌────────┐     ┌──────┐     ┌──────────┐     ┌─────────┐  │
│  │  User  │────>│ Args │────>│  Config  │────>│Validator│  │
│  │ Input  │     │      │     │          │     │         │  │
│  └────────┘     └──────┘     └──────────┘     └────┬────┘  │
│                                                     │        │
│                                              ┌──────v────┐   │
│                                              │Validation │   │
│                                              │  Result   │   │
│                                              └──────┬────┘   │
│                                                     │        │
│                  ┌──────────────────────────────────┘        │
│                  │                                           │
│           ┌──────v──────┐                                    │
│           │  Operation  │                                    │
│           │  Execution  │                                    │
│           └──────┬──────┘                                    │
│                  │                                           │
│        ┌─────────┼─────────┬─────────┐                      │
│        │         │         │         │                      │
│   ┌────v───┐┌───v────┐┌───v────┐┌───v────┐                │
│   │Install ││Uninst- ││  List  ││ Setup  │                │
│   │        ││all     ││        ││        │                │
│   └────┬───┘└───┬────┘└───┬────┘└───┬────┘                │
│        │        │          │         │                      │
│        └────────┴──────────┴─────────┘                      │
│                  │                                           │
│           ┌──────v──────┐                                    │
│           │   Output    │                                    │
│           │   Handler   │                                    │
│           └──────┬──────┘                                    │
│                  │                                           │
│           ┌──────v──────┐                                    │
│           │    User     │                                    │
│           │   Output    │                                    │
│           └─────────────┘                                    │
└──────────────────────────────────────────────────────────────┘
```

## Install Data Flow (Detailed)

```
┌─────────────────────────────────────────────────────────────────┐
│                   Install Operation Data Flow                   │
│                                                                 │
│  CLI Args:                                                      │
│    --project ~/projects/my-tool                                 │
│    --type release                                               │
│    --rename my-tool-v2                                          │
│    --verbose                                                    │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 1: Parse Arguments                                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Args Struct:                                             │  │
│  │   project: Some(PathBuf("~/projects/my-tool"))           │  │
│  │   rename: Some(String("my-tool-v2"))                     │  │
│  │   type: "release"                                        │  │
│  │   verbose: true                                          │  │
│  │   dry_run: false                                         │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 2: Build Configuration                              │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ InstallConfig:                                           │  │
│  │   project_path: "/home/user/projects/my-tool"            │  │
│  │   binary_name: Some("my-tool-v2")                        │  │
│  │   use_debug: false                                       │  │
│  │   verbose: true                                          │  │
│  │   dry_run: false                                         │  │
│  │   test_dir: None                                         │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 3: Create Output Handler                            │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ OutputHandler:                                           │  │
│  │   Box<dyn OutputHandler> = VerboseOutput                 │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 4: Validation                                       │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Validator checks:                                        │  │
│  │   1. project_path exists                                 │  │
│  │      → /home/user/projects/my-tool ✓                     │  │
│  │                                                          │  │
│  │   2. Cargo.toml exists                                   │  │
│  │      → /home/user/projects/my-tool/Cargo.toml ✓          │  │
│  │                                                          │  │
│  │   3. Extract binary name from Cargo.toml                 │  │
│  │      → Read file                                         │  │
│  │      → Parse TOML                                        │  │
│  │      → [package].name = "my-tool" ✓                      │  │
│  │                                                          │  │
│  │   4. Source binary exists                                │  │
│  │      → /home/user/projects/my-tool/                      │  │
│  │        target/release/my-tool ✓                          │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ ValidationResult:                                        │  │
│  │   binary_name: "my-tool"                                 │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 5: Create Installer                                 │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Installer:                                               │  │
│  │   config: &InstallConfig                                 │  │
│  │   binary_name: "my-tool"                                 │  │
│  │   output: VerboseOutput                                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 6: Installation                                     │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Computed Paths:                                          │  │
│  │   source:      /home/user/projects/my-tool/              │  │
│  │                target/release/my-tool                    │  │
│  │                                                          │  │
│  │   dest_dir:    /home/user/.local/softwarewrighter/bin/   │  │
│  │                                                          │  │
│  │   dest_binary: /home/user/.local/softwarewrighter/bin/   │  │
│  │                my-tool-v2                                │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ File Operations:                                         │  │
│  │   1. Create directory                                    │  │
│  │      fs::create_dir_all(dest_dir) → Ok(())               │  │
│  │                                                          │  │
│  │   2. Copy file                                           │  │
│  │      fs::copy(source, dest_binary) → Ok(bytes_copied)    │  │
│  │                                                          │  │
│  │   3. Set permissions                                     │  │
│  │      fs::set_permissions(dest_binary, 0o755) → Ok(())    │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Step 7: Output Result                                    │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Success Output:                                          │  │
│  │   [1/7] Checking project path ... OK                     │  │
│  │   [2/7] Checking Cargo.toml ... OK                       │  │
│  │   [3/7] Parsing Cargo.toml ... binary name: my-tool      │  │
│  │   [4/7] Checking source binary ... OK                    │  │
│  │   [5/7] Creating destination directory ... OK            │  │
│  │   [6/7] Copying binary ... OK                            │  │
│  │   [7/7] Setting executable permissions ... OK            │  │
│  │                                                          │  │
│  │   Successfully installed: my-tool-v2                     │  │
│  │   Location: ~/.local/softwarewrighter/bin/my-tool-v2     │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Uninstall Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                 Uninstall Operation Data Flow                   │
│                                                                 │
│  CLI Args:                                                      │
│    --uninstall my-tool                                          │
│    --verbose                                                    │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Args → Uninstaller                                       │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Uninstaller:                                             │  │
│  │   binary_name: "my-tool"                                 │  │
│  │   verbose: true                                          │  │
│  │   dry_run: false                                         │  │
│  │   output: VerboseOutput                                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Compute Paths:                                           │  │
│  │   dest_dir:    ~/.local/softwarewrighter/bin/            │  │
│  │   binary_path: ~/.local/softwarewrighter/bin/my-tool     │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Validate:                                                │  │
│  │   fs::metadata(binary_path) → exists? ✓                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Remove:                                                  │  │
│  │   fs::remove_file(binary_path) → Ok(())                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Output:                                                  │  │
│  │   Successfully uninstalled: my-tool                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## List Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                   List Operation Data Flow                      │
│                                                                 │
│  CLI Args:                                                      │
│    --list                                                       │
│    --sort newest                                                │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Args → Lister                                            │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Lister:                                                  │  │
│  │   sort_order: SortOrder::Newest                          │  │
│  │   verbose: false                                         │  │
│  │   output: NormalOutput                                   │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Scan Directory:                                          │  │
│  │   dest_dir: ~/.local/softwarewrighter/bin/               │  │
│  │   fs::read_dir(dest_dir) → iterator                      │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Collect Binary Info:                                     │  │
│  │   For each file:                                         │  │
│  │     name: String                                         │  │
│  │     metadata: fs::Metadata                               │  │
│  │     modified: SystemTime                                 │  │
│  │                                                          │  │
│  │   Vec<BinaryInfo> = [                                    │  │
│  │     { name: "ask", modified: 2024-01-15T10:30:00 },      │  │
│  │     { name: "my-tool", modified: 2024-01-16T14:20:00 },  │  │
│  │     { name: "sw-install", modified: 2024-01-17T09:15:00 }│  │
│  │   ]                                                      │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Sort:                                                    │  │
│  │   sort_by: SortOrder::Newest                             │  │
│  │   binaries.sort_by(|a, b| b.modified.cmp(&a.modified))   │  │
│  │                                                          │  │
│  │   Sorted Vec<BinaryInfo> = [                             │  │
│  │     { name: "sw-install", modified: 2024-01-17... },     │  │
│  │     { name: "my-tool", modified: 2024-01-16... },        │  │
│  │     { name: "ask", modified: 2024-01-15... }             │  │
│  │   ]                                                      │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Format Output:                                           │  │
│  │   For each binary:                                       │  │
│  │     compute time_ago(modified)                           │  │
│  │     format: "name (time_ago)"                            │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Output:                                                  │  │
│  │   Installed binaries (sorted by newest):                 │  │
│  │     sw-install (2 hours ago)                             │  │
│  │     my-tool (1 day ago)                                  │  │
│  │     ask (3 days ago)                                     │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Setup Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                   Setup Operation Data Flow                     │
│                                                                 │
│  CLI Args:                                                      │
│    --setup-install-dir                                          │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Args → Setup                                             │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Setup:                                                   │  │
│  │   verbose: false                                         │  │
│  │   output: NormalOutput                                   │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Create Installation Directory:                           │  │
│  │   dest_dir: ~/.local/softwarewrighter/bin/               │  │
│  │   fs::create_dir_all(dest_dir) → Ok(())                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Detect Shell:                                            │  │
│  │   env::var("SHELL") → "/bin/zsh"                         │  │
│  │   shell_type: "zsh"                                      │  │
│  │   config_file: ~/.zshrc                                  │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Check if PATH Already Configured:                        │  │
│  │   read ~/.zshrc                                          │  │
│  │   search for "softwarewrighter/bin"                      │  │
│  │   found: false                                           │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Append PATH Configuration:                               │  │
│  │   config_text:                                           │  │
│  │     "# Added by sw-install\n"                            │  │
│  │     "export PATH=\"$HOME/.local/softwarewrighter/        │  │
│  │                   bin:$PATH\"\n"                         │  │
│  │                                                          │  │
│  │   fs::OpenOptions::new()                                 │  │
│  │     .append(true)                                        │  │
│  │     .open(~/.zshrc)                                      │  │
│  │     .write_all(config_text) → Ok(())                     │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Output:                                                  │  │
│  │   Setup complete!                                        │  │
│  │   Created: ~/.local/softwarewrighter/bin/                │  │
│  │   Updated: ~/.zshrc                                      │  │
│  │                                                          │  │
│  │   To activate, run:                                      │  │
│  │     source ~/.zshrc                                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Error Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Error Propagation Flow                       │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Operation Error:                                         │  │
│  │   Validator::validate()                                  │  │
│  │     fs::metadata(project_path) → Err(NotFound)           │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Wrap in InstallError:                                    │  │
│  │   Err(InstallError::ProjectNotFound(                     │  │
│  │     PathBuf::from("/home/user/projects/missing")         │  │
│  │   ))                                                     │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Propagate with ?:                                        │  │
│  │   let result = validator.validate()?;                    │  │
│  │   // Error bubbles up to caller                          │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Eventually reaches main():                               │  │
│  │   match run_install(args) {                              │  │
│  │     Ok(_) => exit(0),                                    │  │
│  │     Err(e) => {                                          │  │
│  │       eprintln!("Error: {}", e);                         │  │
│  │       exit(1);                                           │  │
│  │     }                                                    │  │
│  │   }                                                      │  │
│  └──────────────┬───────────────────────────────────────────┘  │
│                 v                                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ User sees:                                               │  │
│  │   Error: Project path does not exist:                    │  │
│  │          /home/user/projects/missing                     │  │
│  │                                                          │  │
│  │ Exit code: 1                                             │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Configuration Transformation Flow

```
CLI Arguments          InstallConfig          Computed Paths
─────────────          ─────────────          ──────────────

--project PATH    →    project_path      →    Expanded path
                                              /home/user/...

--rename NAME     →    binary_name       →    Used for dest
                                              filename

--type TYPE       →    use_debug         →    target/[TYPE]/
                       (bool)                 name

--verbose         →    verbose           →    OutputHandler
                       (bool)                 selection

--dry-run         →    dry_run           →    OutputHandler
                       (bool)                 selection

--test-dir DIR    →    test_dir          →    Override dest
                       (Option)               directory
```

## See Also

- [Architecture Overview](Architecture-Overview) - High-level architecture
- [Sequence Diagrams](Sequence-Diagrams) - Execution flow diagrams
- [Component Documentation](Home#component-documentation) - Detailed component docs
