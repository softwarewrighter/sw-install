# Architecture Diagrams

This page contains block diagrams illustrating the architecture of sw-install.

## System Component Diagram

```
┌────────────────────────────────────────────────────────────────┐
│                         sw-install                             │
│                                                                │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │                    main.rs (Entry Point)                  │ │
│  │  • Argument parsing (clap)                                │ │
│  │  • Operation routing                                      │ │
│  │  • Error display                                          │ │
│  └────────────────┬──────────────────────────────────────────┘ │
│                   │                                            │
│       ┌───────────┼───────────┬────────────┬────────────┐     │
│       │           │           │            │            │     │
│       v           v           v            v            v     │
│  ┌────────┐ ┌─────────┐ ┌──────────┐ ┌────────┐ ┌────────┐  │
│  │Install │ │Uninstall│ │  List    │ │ Setup  │ │ Config │  │
│  │        │ │         │ │          │ │        │ │        │  │
│  │ ┌────┐ │ │         │ │ • Scan   │ │ • Init │ │ • Path │  │
│  │ │Val │ │ │ • Check │ │ • Sort   │ │ • Shell│ │  logic │  │
│  │ └─┬──┘ │ │ • Remove│ │ • Format │ │  config│ │        │  │
│  │   │    │ │         │ │          │ │        │ │        │  │
│  │   v    │ │         │ │          │ │        │ │        │  │
│  │ ┌────┐ │ │         │ │          │ │        │ │        │  │
│  │ │Copy│ │ │         │ │          │ │        │ │        │  │
│  │ │Perm│ │ │         │ │          │ │        │ │        │  │
│  │ └────┘ │ │         │ │          │ │        │ │        │  │
│  └────┬───┘ └────┬────┘ └─────┬────┘ └───┬────┘ └────────┘  │
│       │          │            │          │                   │
│       └──────────┴────────────┴──────────┘                   │
│                       │                                       │
│                       v                                       │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │            Output Handler (Trait)                        │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐               │ │
│  │  │  Normal  │  │ Verbose  │  │  DryRun  │               │ │
│  │  └──────────┘  └──────────┘  └──────────┘               │ │
│  └──────────────────────────────────────────────────────────┘ │
│                       │                                       │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │            Error Handler (error.rs)                      │ │
│  │  • InstallError enum                                     │ │
│  │  • User-friendly messages                                │ │
│  └──────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

## Module Dependency Graph

```
┌──────────┐
│ main.rs  │
└────┬─────┘
     │
     ├─────────────┬─────────────┬─────────────┬─────────────┐
     │             │             │             │             │
     v             v             v             v             v
┌─────────┐  ┌──────────┐  ┌────────┐  ┌────────┐  ┌────────┐
│installer│  │uninstaller│  │ lister │  │ setup  │  │ config │
└────┬────┘  └─────┬────┘  └───┬────┘  └───┬────┘  └───┬────┘
     │             │            │           │           │
     └─────────────┴────────────┴───────────┴───────────┘
                   │
                   v
         ┌─────────────────────┐
         │   Common Modules    │
         ├─────────────────────┤
         │ • validator.rs      │
         │ • output.rs         │
         │ • error.rs          │
         │ • config.rs         │
         └─────────────────────┘
```

## Installer Component Architecture

```
┌─────────────────────────────────────────────────────┐
│                Installer Component                  │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │         Validator (Pre-flight)          │       │
│  ├─────────────────────────────────────────┤       │
│  │ 1. Check project path exists            │       │
│  │ 2. Check Cargo.toml exists              │       │
│  │ 3. Parse Cargo.toml → binary name       │       │
│  │ 4. Check binary exists in target/       │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│                 v                                   │
│  ┌─────────────────────────────────────────┐       │
│  │     Installer (File Operations)         │       │
│  ├─────────────────────────────────────────┤       │
│  │ 1. Create destination directory         │       │
│  │    └─> ~/.local/softwarewrighter/bin/   │       │
│  │                                          │       │
│  │ 2. Copy binary                           │       │
│  │    source: target/[release|debug]/name   │       │
│  │    dest:   ~/.local/.../bin/[name]       │       │
│  │                                          │       │
│  │ 3. Set executable permissions            │       │
│  │    chmod +x (Unix)                       │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│                 v                                   │
│  ┌─────────────────────────────────────────┐       │
│  │      Output Handler (Reporting)         │       │
│  ├─────────────────────────────────────────┤       │
│  │ • Success message                        │       │
│  │ • Installation location                  │       │
│  │ • Step-by-step details (if verbose)      │       │
│  └─────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────┘
```

## Output Handler Strategy Pattern

```
┌────────────────────────────────────────────────────┐
│              OutputHandler Trait                   │
│  ┌──────────────────────────────────────────────┐ │
│  │ trait OutputHandler {                        │ │
│  │   fn info(&self, msg: &str);                │ │
│  │   fn step(&self, msg: &str);                │ │
│  │   fn success(&self, msg: &str);             │ │
│  │   fn error(&self, msg: &str);               │ │
│  │ }                                            │ │
│  └──────────────────────────────────────────────┘ │
└─────────────────┬──────────────────────────────────┘
                  │
      ┌───────────┼───────────┐
      │           │           │
      v           v           v
┌──────────┐ ┌─────────┐ ┌─────────┐
│  Normal  │ │ Verbose │ │ DryRun  │
│  Output  │ │ Output  │ │ Output  │
├──────────┤ ├─────────┤ ├─────────┤
│ • Minimal│ │ • Step  │ │ • Prefix│
│   output │ │   by    │ │   all   │
│ • Success│ │   step  │ │   with  │
│   and    │ │ • Paths │ │  "Would:"│
│   errors │ │ • Status│ │ • No    │
│   only   │ │ • Count │ │   exec  │
└──────────┘ └─────────┘ └─────────┘
```

## Path Resolution Flow

```
┌─────────────────────────────────────────────────────┐
│              Path Resolution                        │
│                                                     │
│  User Input:                                        │
│    --project ~/projects/my-tool                     │
│    --type release                                   │
│    --rename my-tool-v2                              │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │        Configuration Layer              │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│  ┌──────────────v──────────────────────────┐       │
│  │ project_path (expand ~)                 │       │
│  │   ~/projects/my-tool                    │       │
│  │   → /home/user/projects/my-tool         │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│  ┌──────────────v──────────────────────────┐       │
│  │ cargo_toml_path                         │       │
│  │   {project_path}/Cargo.toml             │       │
│  │   → /home/user/projects/my-tool/        │       │
│  │     Cargo.toml                          │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│  ┌──────────────v──────────────────────────┐       │
│  │ source_binary_path                      │       │
│  │   {project_path}/target/{type}/         │       │
│  │   {binary_name}                         │       │
│  │   → /home/user/projects/my-tool/        │       │
│  │     target/release/my-tool              │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│  ┌──────────────v──────────────────────────┐       │
│  │ destination_dir                         │       │
│  │   ~/.local/softwarewrighter/bin/        │       │
│  │   → /home/user/.local/                  │       │
│  │     softwarewrighter/bin/               │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│  ┌──────────────v──────────────────────────┐       │
│  │ destination_binary_path                 │       │
│  │   {destination_dir}/{rename or name}    │       │
│  │   → /home/user/.local/                  │       │
│  │     softwarewrighter/bin/my-tool-v2     │       │
│  └─────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────┘
```

## Error Handling Architecture

```
┌─────────────────────────────────────────────────────┐
│              Error Handling Flow                    │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │         Operation Layer                 │       │
│  │  (Installer/Uninstaller/Validator)      │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│                 │ Returns Result<T, InstallError>   │
│                 v                                   │
│  ┌─────────────────────────────────────────┐       │
│  │      InstallError Enum                  │       │
│  ├─────────────────────────────────────────┤       │
│  │ • ProjectNotFound(PathBuf)              │       │
│  │ • CargoTomlNotFound(PathBuf)            │       │
│  │ • BinaryNotFound(PathBuf)               │       │
│  │ • InvalidBinaryName(String)             │       │
│  │ • HomeNotFound                          │       │
│  │ • Io(std::io::Error)                    │       │
│  │ • NoOperationSpecified                  │       │
│  └──────────────┬──────────────────────────┘       │
│                 │                                   │
│                 │ Error propagates up               │
│                 v                                   │
│  ┌─────────────────────────────────────────┐       │
│  │           main.rs                       │       │
│  ├─────────────────────────────────────────┤       │
│  │ match result {                          │       │
│  │   Ok(_) => exit(0),                     │       │
│  │   Err(e) => {                           │       │
│  │     eprintln!("Error: {}", e);          │       │
│  │     exit(1);                            │       │
│  │   }                                     │       │
│  │ }                                       │       │
│  └─────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────┘
```

## Testing Architecture

```
┌─────────────────────────────────────────────────────┐
│              Testing Layers                         │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │         Unit Tests                      │       │
│  ├─────────────────────────────────────────┤       │
│  │ config.rs:                              │       │
│  │   • Path computations                   │       │
│  │   • Rename logic                        │       │
│  │                                          │       │
│  │ validator.rs:                            │       │
│  │   • Project validation                   │       │
│  │   • Cargo.toml parsing                   │       │
│  │                                          │       │
│  │ installer.rs:                            │       │
│  │   • Directory creation                   │       │
│  │   • File copying                         │       │
│  │   • Permission setting                   │       │
│  │                                          │       │
│  │ output.rs:                               │       │
│  │   • Output formatting                    │       │
│  │   • Mode switching                       │       │
│  └─────────────────────────────────────────┘       │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │      Integration Tests                  │       │
│  ├─────────────────────────────────────────┤       │
│  │ • Full install workflow                  │       │
│  │ • Uninstall workflow                     │       │
│  │ • List workflow                          │       │
│  │ • Setup workflow                         │       │
│  │ • Dry-run mode                           │       │
│  │ • Error scenarios                        │       │
│  └─────────────────────────────────────────┘       │
│                                                     │
│  ┌─────────────────────────────────────────┐       │
│  │      Test Utilities                     │       │
│  ├─────────────────────────────────────────┤       │
│  │ • Temporary directory creation           │       │
│  │ • Mock Cargo projects                    │       │
│  │ • Test output handlers                   │       │
│  └─────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────┘
```

## See Also

- [Architecture Overview](Architecture-Overview) - High-level architecture description
- [Sequence Diagrams](Sequence-Diagrams) - Execution flow diagrams
- [Component Documentation](Home#component-documentation) - Detailed component docs
