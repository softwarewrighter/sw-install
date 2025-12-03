# Refactoring Plan for sw-checklist Compliance

## Current Status
sw-checklist reports: 10 passed, 13 failed, 25 warnings

## Issues to Fix

### Critical (FAIL)

#### 1. Rust Edition (Priority: HIGH)
**Issue**: Using Rust 2021 edition, must use 2024
**Fix**: Update `Cargo.toml` edition field
```toml
edition = "2024"
```
**Effort**: Minimal - may require fixing 2024-specific lints

#### 2. File LOC: validator.rs (691 lines, max 500)
**Issue**: validator.rs is too large
**Fix**: Split into submodules:
- `validator/mod.rs` - Core Validator struct and validate()
- `validator/project_type.rs` - ProjectType enum and detection
- `validator/workspace.rs` - Workspace binary scanning
- `validator/simple.rs` - Simple project handling

#### 3. Module Function Count (Multiple modules exceed 7 functions)
This is the most significant issue. Current counts:
- validator.rs: 23 functions
- lister.rs: 24 functions
- output.rs: 22 functions
- config.rs: 14 functions
- setup.rs: 14 functions
- installer.rs: 10 functions
- uninstaller.rs: 10 functions

**Fix Strategy**: Many of these functions are tests. Consider:
1. Move tests to separate test files in `tests/` directory
2. Split modules with clear single responsibilities
3. Use submodules to organize related functions

#### 4. Crate Module Count (10 modules, max 7)
Current modules: error, output, config, validator, installer, uninstaller, lister, setup, lib, main

**Fix Strategy**:
- Group related modules into parent modules
- Example structure:
  ```
  src/
    lib.rs
    main.rs
    error.rs           (keep)
    output.rs          (keep)
    config.rs          (keep)
    operations/        (new parent module)
      mod.rs
      installer.rs
      uninstaller.rs
      lister.rs
      setup.rs
    validation/        (new parent module)
      mod.rs
      validator.rs
      project_type.rs
  ```

#### 5. Function LOC Failures
- `test_workspace_with_library_only_members_ignores_libs`: 60 lines (max 50)
- `test_multi_component_project_detection`: 52 lines (max 50)
- `list` in lister.rs: 59 lines (max 50)

**Fix**: Break into smaller helper functions

### Warnings (25 total)

Most warnings are for functions 25-50 lines. These are acceptable but worth reviewing:
- Consider extracting helper functions for complex logic
- Test functions can use setup helpers to reduce boilerplate

## Recommended Implementation Order

### Phase 1: Rust Edition Update
1. Update Cargo.toml to edition = "2024"
2. Run cargo build and fix any 2024-specific issues
3. Run cargo clippy and fix new warnings
4. Run tests

### Phase 2: Split Large Functions
1. Split `list()` in lister.rs into smaller functions
2. Refactor large test functions using test helpers

### Phase 3: Module Reorganization
1. Create `src/operations/` directory
2. Move installer.rs, uninstaller.rs, lister.rs, setup.rs
3. Create `src/validation/` directory
4. Split validator.rs into submodules
5. Update lib.rs exports
6. Update main.rs imports
7. Run tests to verify

### Phase 4: Test Organization
1. Move unit tests from individual modules to `tests/` where appropriate
2. Create test helper modules for common setup
3. Ensure all tests still pass

## Impact Assessment

- **Code Changes**: Significant restructuring
- **API Changes**: None (internal reorganization only)
- **Risk**: Medium - tests should catch regressions
- **Effort**: 2-4 hours

## Validation

After each phase:
1. Run `cargo test`
2. Run `cargo clippy`
3. Run `cargo fmt`
4. Run `sw-checklist` to verify improvements
