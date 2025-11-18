# Sequence Diagrams

This page contains sequence diagrams illustrating the execution flow for key operations in sw-install.

## Install Operation Flow

```
User         CLI          Config      Validator    Installer    OutputHandler    FileSystem
 │            │             │            │            │              │              │
 │  install   │             │            │            │              │              │
 │───────────>│             │            │            │              │              │
 │            │ parse args  │            │            │              │              │
 │            │────────────>│            │            │              │              │
 │            │             │            │            │              │              │
 │            │   build     │            │            │              │              │
 │            │   config    │            │            │              │              │
 │            │<────────────│            │            │              │              │
 │            │             │            │            │              │              │
 │            │         create validator │            │              │              │
 │            │─────────────────────────>│            │              │              │
 │            │             │            │            │              │              │
 │            │             │   validate project path │              │              │
 │            │             │            │────────────────────────────────────────>│
 │            │             │            │            │              │    exists?  │
 │            │             │            │<────────────────────────────────────────│
 │            │             │            │            │              │     yes     │
 │            │             │            │            │              │              │
 │            │             │   validate Cargo.toml   │              │              │
 │            │             │            │────────────────────────────────────────>│
 │            │             │            │            │              │   exists?   │
 │            │             │            │<────────────────────────────────────────│
 │            │             │            │            │              │     yes     │
 │            │             │            │            │              │              │
 │            │             │   parse Cargo.toml      │              │              │
 │            │             │            │────────────────────────────────────────>│
 │            │             │            │            │              │   read file │
 │            │             │            │<────────────────────────────────────────│
 │            │             │            │            │              │   contents  │
 │            │             │  extract   │            │              │              │
 │            │             │binary name │            │              │              │
 │            │             │            │            │              │              │
 │            │             │   validate source binary│              │              │
 │            │             │            │────────────────────────────────────────>│
 │            │             │            │            │              │   exists?   │
 │            │             │            │<────────────────────────────────────────│
 │            │             │            │            │              │     yes     │
 │            │             │            │            │              │              │
 │            │             │ validation │            │              │              │
 │            │             │   result   │            │              │              │
 │            │<────────────────────────────────────  │              │              │
 │            │             │            │            │              │              │
 │            │         create installer │            │              │              │
 │            │────────────────────────────────────────>             │              │
 │            │             │            │            │              │              │
 │            │             │            │  install() │              │              │
 │            │             │            │            │              │              │
 │            │             │            │   create destination dir  │              │
 │            │             │            │            │──────────────>              │
 │            │             │            │            │     step     │              │
 │            │             │            │            │              │              │
 │            │             │            │            │────────────────────────────>│
 │            │             │            │            │              │  mkdir -p   │
 │            │             │            │            │<────────────────────────────│
 │            │             │            │            │              │   success   │
 │            │             │            │            │              │              │
 │            │             │            │    copy binary            │              │
 │            │             │            │            │──────────────>              │
 │            │             │            │            │     step     │              │
 │            │             │            │            │              │              │
 │            │             │            │            │────────────────────────────>│
 │            │             │            │            │              │ copy file   │
 │            │             │            │            │<────────────────────────────│
 │            │             │            │            │              │   success   │
 │            │             │            │            │              │              │
 │            │             │            │  set permissions          │              │
 │            │             │            │            │──────────────>              │
 │            │             │            │            │     step     │              │
 │            │             │            │            │              │              │
 │            │             │            │            │────────────────────────────>│
 │            │             │            │            │              │  chmod +x   │
 │            │             │            │            │<────────────────────────────│
 │            │             │            │            │              │   success   │
 │            │             │            │            │              │              │
 │            │             │            │            │──────────────>              │
 │            │             │            │            │   success    │              │
 │            │             │            │            │              │              │
 │            │             │            │  result    │              │              │
 │            │<──────────────────────────────────────────────────── │              │
 │            │             │            │            │              │              │
 │  success   │             │            │            │              │              │
 │<───────────│             │            │            │              │              │
```

## Uninstall Operation Flow

```
User         CLI      Uninstaller    OutputHandler    FileSystem
 │            │            │              │              │
 │ uninstall  │            │              │              │
 │───────────>│            │              │              │
 │            │ parse args │              │              │
 │            │            │              │              │
 │            │  create    │              │              │
 │            │uninstaller │              │              │
 │            │───────────>│              │              │
 │            │            │              │              │
 │            │ uninstall()│              │              │
 │            │            │              │              │
 │            │    compute destination   │              │
 │            │      directory path      │              │
 │            │            │              │              │
 │            │    validate binary exists│              │
 │            │            │──────────────>              │
 │            │            │     step     │              │
 │            │            │              │              │
 │            │            │────────────────────────────>│
 │            │            │              │   exists?   │
 │            │            │<────────────────────────────│
 │            │            │              │     yes     │
 │            │            │              │              │
 │            │    remove binary          │              │
 │            │            │──────────────>              │
 │            │            │     step     │              │
 │            │            │              │              │
 │            │            │────────────────────────────>│
 │            │            │              │  remove file│
 │            │            │<────────────────────────────│
 │            │            │              │   success   │
 │            │            │              │              │
 │            │            │──────────────>              │
 │            │            │   success    │              │
 │            │            │              │              │
 │            │   result   │              │              │
 │            │<───────────│              │              │
 │            │            │              │              │
 │  success   │            │              │              │
 │<───────────│            │              │              │
```

## List Operation Flow

```
User         CLI        Lister     OutputHandler    FileSystem
 │            │           │             │              │
 │   list     │           │             │              │
 │───────────>│           │             │              │
 │            │ parse args│             │              │
 │            │           │             │              │
 │            │  create   │             │              │
 │            │  lister   │             │              │
 │            │──────────>│             │              │
 │            │           │             │              │
 │            │   list()  │             │              │
 │            │           │             │              │
 │            │  compute destination    │              │
 │            │      directory          │              │
 │            │           │             │              │
 │            │  read directory         │              │
 │            │           │─────────────────────────────>
 │            │           │             │  list files │
 │            │           │<────────────────────────────│
 │            │           │             │ [file1, ...] │
 │            │           │             │              │
 │            │  get metadata for each  │              │
 │            │      file in list       │              │
 │            │           │─────────────────────────────>
 │            │           │             │get modified │
 │            │           │<────────────────────────────│
 │            │           │             │  timestamp  │
 │            │           │             │              │
 │            │   sort by option        │              │
 │            │  (name/newest/oldest)   │              │
 │            │           │             │              │
 │            │  format output          │              │
 │            │           │             │              │
 │            │           │ for each binary:           │
 │            │           │────────────>│              │
 │            │           │   format    │              │
 │            │           │   line      │              │
 │            │           │             │              │
 │            │  result   │             │              │
 │            │<──────────│             │              │
 │            │           │             │              │
 │   output   │           │             │              │
 │<───────────│           │             │              │
```

## Setup Operation Flow

```
User         CLI        Setup      OutputHandler    FileSystem
 │            │           │             │              │
 │   setup    │           │             │              │
 │───────────>│           │             │              │
 │            │ parse args│             │              │
 │            │           │             │              │
 │            │  create   │             │              │
 │            │  setup    │             │              │
 │            │──────────>│             │              │
 │            │           │             │              │
 │            │  setup()  │             │              │
 │            │           │             │              │
 │            │  create installation    │              │
 │            │      directory          │              │
 │            │           │────────────>│              │
 │            │           │    info     │              │
 │            │           │             │              │
 │            │           │─────────────────────────────>
 │            │           │             │   mkdir -p  │
 │            │           │<────────────────────────────│
 │            │           │             │   success   │
 │            │           │             │              │
 │            │  detect shell type      │              │
 │            │ (SHELL env variable)    │              │
 │            │           │             │              │
 │            │  determine shell config │              │
 │            │    file (~/.bashrc or   │              │
 │            │        ~/.zshrc)        │              │
 │            │           │             │              │
 │            │  check if PATH already  │              │
 │            │      configured         │              │
 │            │           │─────────────────────────────>
 │            │           │             │  read file  │
 │            │           │<────────────────────────────│
 │            │           │             │  contents   │
 │            │           │             │              │
 │            │  if not configured:     │              │
 │            │  append PATH config     │              │
 │            │           │────────────>│              │
 │            │           │    step     │              │
 │            │           │             │              │
 │            │           │─────────────────────────────>
 │            │           │             │append to file│
 │            │           │<────────────────────────────│
 │            │           │             │   success   │
 │            │           │             │              │
 │            │           │────────────>│              │
 │            │           │   success   │              │
 │            │           │   message   │              │
 │            │           │             │              │
 │            │  provide instructions   │              │
 │            │  to reload shell        │              │
 │            │           │────────────>│              │
 │            │           │    info     │              │
 │            │           │             │              │
 │            │  result   │             │              │
 │            │<──────────│             │              │
 │            │           │             │              │
 │instructions│           │             │              │
 │<───────────│           │             │              │
```

## Validation Flow (Detail)

```
Validator    Config      FileSystem    Cargo.toml
    │           │             │            │
    │validate() │             │            │
    │           │             │            │
    │  get project path       │            │
    │──────────>│             │            │
    │<──────────│             │            │
    │           │             │            │
    │  check exists           │            │
    │─────────────────────────>            │
    │         path exists?    │            │
    │<─────────────────────────            │
    │           │      yes    │            │
    │           │             │            │
    │  get Cargo.toml path    │            │
    │──────────>│             │            │
    │<──────────│             │            │
    │           │             │            │
    │  check exists           │            │
    │─────────────────────────>            │
    │         file exists?    │            │
    │<─────────────────────────            │
    │           │      yes    │            │
    │           │             │            │
    │  read Cargo.toml        │            │
    │─────────────────────────>            │
    │         file contents   │            │
    │<─────────────────────────            │
    │           │             │            │
    │  parse TOML             │            │
    │─────────────────────────────────────>│
    │           │             │   parse    │
    │           │             │            │
    │  extract [package].name │            │
    │<─────────────────────────────────────│
    │           │             │binary_name │
    │           │             │            │
    │  get source binary path │            │
    │──────────>│             │            │
    │  (target/release/name)  │            │
    │<──────────│             │            │
    │           │             │            │
    │  check binary exists    │            │
    │─────────────────────────>            │
    │         file exists?    │            │
    │<─────────────────────────            │
    │           │      yes    │            │
    │           │             │            │
    │  return ValidationResult│            │
    │  { binary_name }        │            │
    │           │             │            │
```

## Output Handler Selection Flow

```
Main         Args        OutputHandler
 │            │               │
 │  parse     │               │
 │───────────>│               │
 │            │               │
 │ get flags  │               │
 │<───────────│               │
 │  verbose?  │               │
 │  dry_run?  │               │
 │            │               │
 │  create_output_handler()   │
 │            │               │
 │  if dry_run == true        │
 │───────────────────────────>│
 │         DryRunOutput       │
 │            │               │
 │  else if verbose == true   │
 │───────────────────────────>│
 │        VerboseOutput       │
 │            │               │
 │  else                      │
 │───────────────────────────>│
 │        NormalOutput        │
 │            │               │
 │  return Box<dyn Output>    │
 │<───────────────────────────│
```

## Error Propagation Flow

```
Operation    Validator    Installer    Main         User
    │            │            │         │            │
    │  validate()│            │         │            │
    │───────────>│            │         │            │
    │            │ check path │         │            │
    │            │  not found!│         │            │
    │            │            │         │            │
    │ Err(       │            │         │            │
    │ ProjectNot │            │         │            │
    │ Found)     │            │         │            │
    │<───────────│            │         │            │
    │            │            │         │            │
    │  propagate error with ? │         │            │
    │────────────────────────────────────>           │
    │            │            │         │            │
    │            │            │  match Err(e)        │
    │            │            │         │            │
    │            │            │  eprintln!("Error")  │
    │            │            │         │            │
    │            │            │  display error       │
    │            │            │         │───────────>│
    │            │            │         │  "Error:   │
    │            │            │         │   Project  │
    │            │            │         │   path does│
    │            │            │         │   not      │
    │            │            │         │   exist"   │
    │            │            │         │            │
    │            │            │  exit(1)│            │
```

## Dry-Run Mode Flow

```
User         CLI      Installer    DryRunOutput    FileSystem
 │            │           │             │              │
 │ install    │           │             │              │
 │ --dry-run  │           │             │              │
 │───────────>│           │             │              │
 │            │           │             │              │
 │            │  output = DryRunOutput  │              │
 │            │           │             │              │
 │            │ install() │             │              │
 │            │──────────>│             │              │
 │            │           │             │              │
 │            │  create_destination_dir │              │
 │            │           │             │              │
 │            │           │────────────>│              │
 │            │           │ step("Would:│              │
 │            │           │ create...")  │              │
 │            │           │             │              │
 │            │           │ NO ACTUAL   │              │
 │            │           │ FILESYSTEM  │              │
 │            │           │ OPERATION   │              │
 │            │           │             │              │
 │            │  copy_binary            │              │
 │            │           │             │              │
 │            │           │────────────>│              │
 │            │           │ step("Would:│              │
 │            │           │ copy...")    │              │
 │            │           │             │              │
 │            │           │ NO ACTUAL   │              │
 │            │           │ COPY        │              │
 │            │           │             │              │
 │            │  set_permissions        │              │
 │            │           │             │              │
 │            │           │────────────>│              │
 │            │           │ step("Would:│              │
 │            │           │ chmod...")   │              │
 │            │           │             │              │
 │            │           │ NO ACTUAL   │              │
 │            │           │ CHMOD       │              │
 │            │           │             │              │
 │            │  success  │             │              │
 │            │<──────────│             │              │
 │            │           │             │              │
 │  "Dry-run  │           │             │              │
 │  complete" │           │             │              │
 │<───────────│           │             │              │
```

## See Also

- [Architecture Overview](Architecture-Overview) - High-level architecture
- [Architecture Diagrams](Architecture-Diagrams) - Component diagrams
- [Data Flow](Data-Flow) - Data flow through the system
