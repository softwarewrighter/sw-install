# Implementation Plan

## sw-install

### Phase 1: Project Setup and Foundation (Complete this phase first)

#### 1.1 Documentation
- [x] Create docs/ directory structure
- [x] Write PRD (Product Requirements Document)
- [x] Write architecture.md
- [x] Write design.md
- [x] Write process.md
- [ ] Write plan.md (this file)
- [ ] Write status.md

#### 1.2 Project Configuration
- [ ] Update Cargo.toml with dependencies
  - clap (CLI parsing with derive)
  - toml (parse Cargo.toml files)
  - thiserror (error handling)
  - tempfile (for tests)
- [ ] Create .gitignore
- [ ] Create src/lib.rs for testable code
- [ ] Create tests/integration.rs structure

#### 1.3 Pre-commit Infrastructure
- [ ] Create scripts/pre-commit-check.sh
- [ ] Make script executable
- [ ] Test pre-commit script
- [ ] Document usage in README.md

### Phase 2: Core Infrastructure (TDD - Foundation)

#### 2.1 Error Handling
- [ ] Write error.rs with InstallError enum
- [ ] Test error display messages
- [ ] Test error conversions (From implementations)

#### 2.2 Output Handling
- [ ] Write tests for OutputHandler trait
- [ ] Implement OutputHandler trait
- [ ] Test NormalOutput implementation
- [ ] Implement NormalOutput
- [ ] Test VerboseOutput implementation
- [ ] Implement VerboseOutput
- [ ] Test DryRunOutput implementation
- [ ] Implement DryRunOutput
- [ ] Test create_output_handler factory

#### 2.3 Configuration Module
- [ ] Write tests for InstallConfig::new()
- [ ] Implement InstallConfig struct
- [ ] Write tests for destination_dir()
- [ ] Implement destination_dir()
- [ ] Write tests for source_binary_path()
- [ ] Implement source_binary_path()
- [ ] Write tests for destination_binary_path()
- [ ] Implement destination_binary_path()
- [ ] Write tests for target_subdir()
- [ ] Implement target_subdir()

### Phase 3: Validation (TDD - Core Logic)

#### 3.1 Project Path Validation
- [ ] Write test: validator fails when project path missing
- [ ] Implement: validate_project_path()
- [ ] Write test: validator fails when path is not a directory
- [ ] Fix implementation

#### 3.2 Cargo.toml Validation
- [ ] Write test: validator fails when Cargo.toml missing
- [ ] Implement: validate_cargo_toml()
- [ ] Write test: validator extracts binary name correctly
- [ ] Implement: extract_binary_name() with toml parsing
- [ ] Write test: validator handles multi-binary projects
- [ ] Fix implementation for multiple [[bin]] entries

#### 3.3 Binary Validation
- [ ] Write test: validator fails when binary not built
- [ ] Implement: validate_source_binary()
- [ ] Write test: validator checks debug vs release correctly
- [ ] Fix implementation for debug/release paths

#### 3.4 Full Validation Flow
- [ ] Write test: validator.validate() returns ValidationResult
- [ ] Implement: Validator::validate() orchestration
- [ ] Write test: validation with verbose output
- [ ] Write test: validation with dry-run output

### Phase 4: Installation (TDD - Core Operations)

#### 4.1 Directory Creation
- [ ] Write test: create_destination_dir() makes nested directories
- [ ] Implement: create_destination_dir()
- [ ] Write test: dry-run doesn't create directories
- [ ] Fix implementation for dry-run mode
- [ ] Write test: existing directory doesn't cause error
- [ ] Fix implementation to handle existing directories

#### 4.2 Binary Copy
- [ ] Write test: copy_binary() copies file correctly
- [ ] Implement: copy_binary()
- [ ] Write test: dry-run doesn't copy file
- [ ] Fix implementation for dry-run mode
- [ ] Write test: rename option changes destination name
- [ ] Fix implementation for rename

#### 4.3 Permissions
- [ ] Write test: set_permissions() makes binary executable
- [ ] Implement: set_permissions() with Unix chmod
- [ ] Write test: dry-run doesn't modify permissions
- [ ] Fix implementation for dry-run mode

#### 4.4 Full Installation Flow
- [ ] Write test: installer.install() performs full installation
- [ ] Implement: Installer::install() orchestration
- [ ] Write test: installation with verbose output
- [ ] Write test: installation with dry-run output

### Phase 5: CLI Interface (TDD - User Interaction)

#### 5.1 Argument Parsing
- [ ] Write test: parse --project argument
- [ ] Implement: Args struct with clap
- [ ] Write test: parse --rename argument
- [ ] Fix implementation
- [ ] Write test: parse --type debug flag
- [ ] Fix implementation
- [ ] Write test: parse --verbose flag
- [ ] Fix implementation
- [ ] Write test: parse --dry-run flag
- [ ] Fix implementation
- [ ] Write test: missing required argument shows error
- [ ] Verify clap handles this

#### 5.2 Main Function
- [ ] Write test: main creates correct InstallConfig from args
- [ ] Implement: main.rs orchestration
- [ ] Write test: main handles validation errors gracefully
- [ ] Fix error handling
- [ ] Write test: main handles installation errors gracefully
- [ ] Fix error handling
- [ ] Write test: main shows success message
- [ ] Fix implementation

### Phase 6: Integration Testing

#### 6.1 End-to-End Tests
- [ ] Create test fixture: minimal Cargo project
- [ ] Write test: install release binary
- [ ] Write test: install debug binary
- [ ] Write test: install with rename
- [ ] Write test: dry-run doesn't modify filesystem
- [ ] Write test: verbose shows all steps
- [ ] Write test: error when project missing
- [ ] Write test: error when Cargo.toml missing
- [ ] Write test: error when binary not built

#### 6.2 Edge Cases
- [ ] Write test: project path with spaces
- [ ] Write test: binary name with hyphens/underscores
- [ ] Write test: destination already exists (overwrite)
- [ ] Write test: invalid rename (special characters)
- [ ] Write test: HOME environment variable not set

### Phase 7: Polish and Documentation

#### 7.1 Code Quality
- [ ] Run cargo fmt
- [ ] Run cargo clippy and fix all warnings
- [ ] Ensure all tests pass
- [ ] Review error messages for clarity
- [ ] Add doc comments to public APIs

#### 7.2 Documentation
- [ ] Write comprehensive README.md
- [ ] Add usage examples to README
- [ ] Add troubleshooting section to README
- [ ] Update status.md with completion status
- [ ] Verify all docs are ASCII-only UTF-8

#### 7.3 Pre-commit Validation
- [ ] Run complete pre-commit checklist
- [ ] Test pre-commit script
- [ ] Fix any issues found
- [ ] Verify .gitignore is complete

### Phase 8: Release Preparation

#### 8.1 Final Testing
- [ ] Test on actual softwarewrighter/ask project
- [ ] Test installation flow end-to-end
- [ ] Verify installed binary works
- [ ] Test all command-line options

#### 8.2 Documentation Review
- [ ] Review all docs for accuracy
- [ ] Check for broken references
- [ ] Ensure consistent terminology
- [ ] Proofread for typos

#### 8.3 Release
- [ ] Tag version 0.1.0
- [ ] Build release binary
- [ ] Test release binary
- [ ] Create release notes

### Success Criteria

Each phase is complete when:
1. All tests pass (cargo test)
2. No clippy warnings (cargo clippy)
3. Code is formatted (cargo fmt)
4. Documentation is updated
5. Status.md reflects progress

### Estimation

- Phase 1: 30 minutes (Documentation setup)
- Phase 2: 1 hour (Core infrastructure)
- Phase 3: 1.5 hours (Validation logic)
- Phase 4: 1.5 hours (Installation logic)
- Phase 5: 1 hour (CLI interface)
- Phase 6: 1 hour (Integration tests)
- Phase 7: 30 minutes (Polish)
- Phase 8: 30 minutes (Release prep)

**Total estimated time: 7-8 hours**

### Current Status
See status.md for real-time progress tracking.
