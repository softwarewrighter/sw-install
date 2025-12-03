# Refactoring Plan for sw-checklist Compliance

## Current State Analysis

### sw-checklist Results (Current)
- **Passed**: 11
- **Failed**: 8 (all module/crate function counts)
- **Warnings**: 20

### Current Structure
```
src/
├── main.rs         (6 functions, 353 lines)
├── lib.rs          (re-exports only)
├── config.rs       (14 functions - FAIL)
├── error.rs        (6 functions - WARN)
├── output.rs       (22 functions - FAIL)
├── validator.rs    (12 functions - FAIL)
├── installer.rs    (10 functions - FAIL)
├── uninstaller.rs  (10 functions - FAIL)
├── lister.rs       (27 functions - FAIL)
├── setup.rs        (14 functions - FAIL)
tests/
└── validator_tests.rs (6 tests)
```

**Module Count**: 10 modules (max 7) - FAIL
**Function Counts**: Most modules exceed 7 function limit due to embedded tests

---

## Target Architecture

Based on sw-checklist and alltalk-client-rs patterns:

### Key Architectural Principles

1. **Separate tests from implementation** - Move ALL tests to `tests/` directory
2. **Small focused modules** - Max 7 functions per module, <500 lines per file
3. **Layered dependencies** - Clear hierarchy with minimal coupling
4. **Pure functions** - Separate logic from I/O where possible
5. **lib.rs contains only re-exports** - No function implementations

### Target Structure

```
src/
├── main.rs              (CLI parsing + orchestration, ~6 functions)
├── lib.rs               (re-exports only, 0 functions)
├── error.rs             (error types, ~2 functions)
├── output.rs            (output handlers, ~4 functions)
├── config.rs            (InstallConfig, ~5 functions)
├── validation/
│   ├── mod.rs           (re-exports, 0 functions)
│   ├── validator.rs     (Validator struct + validate(), ~4 functions)
│   ├── project_type.rs  (ProjectType detection, ~3 functions)
│   └── workspace.rs     (workspace binary scanning, ~3 functions)
├── operations/
│   ├── mod.rs           (re-exports, 0 functions)
│   ├── installer.rs     (Installer, ~4 functions)
│   ├── uninstaller.rs   (Uninstaller, ~4 functions)
│   ├── lister.rs        (Lister, ~5 functions)
│   └── setup.rs         (Setup, ~5 functions)
tests/
├── config_tests.rs
├── error_tests.rs
├── output_tests.rs
├── validator_tests.rs
├── installer_tests.rs
├── uninstaller_tests.rs
├── lister_tests.rs
├── setup_tests.rs
└── test_helpers.rs      (shared test utilities)
```

**Module Count**: 7 (main.rs not counted as module in lib)
- lib.rs, error.rs, output.rs, config.rs, validation/, operations/

---

## Implementation Phases

### Phase 1: Move All Tests to Integration Tests
**Goal**: Reduce function counts in all modules

1. Create `tests/test_helpers.rs` with shared test utilities
2. Move tests from each module to corresponding `tests/*_tests.rs`
3. Remove `#[cfg(test)] mod tests` from all source files
4. Verify all tests pass

**Expected Results**:
- config.rs: 14 → 5 functions
- output.rs: 22 → 4 functions
- validator.rs: 12 → 12 functions (already moved)
- installer.rs: 10 → 5 functions
- uninstaller.rs: 10 → 6 functions
- lister.rs: 27 → 7 functions
- setup.rs: 14 → 7 functions

### Phase 2: Split Validator into Submodules
**Goal**: Reduce validator.rs function count and improve organization

1. Create `src/validation/` directory
2. Move ProjectType enum and detection to `project_type.rs`
3. Move workspace scanning to `workspace.rs`
4. Keep Validator struct and validate() in `validator.rs`
5. Create `mod.rs` with re-exports

**Expected Results**:
- validation/validator.rs: ~4 functions
- validation/project_type.rs: ~3 functions
- validation/workspace.rs: ~3 functions

### Phase 3: Split Operations into Submodules
**Goal**: Organize operations and reduce function counts

1. Create `src/operations/` directory
2. Move installer.rs, uninstaller.rs, lister.rs, setup.rs
3. Create `mod.rs` with re-exports
4. Split lister.rs if still over limit (format_time_ago → time_utils.rs)

**Expected Results**:
- operations/installer.rs: ~4 functions
- operations/uninstaller.rs: ~4 functions
- operations/lister.rs: ~5 functions
- operations/setup.rs: ~5 functions

### Phase 4: Reduce output.rs Function Count
**Goal**: Split output handlers

1. Move VerboseOutput, DryRunOutput, VerboseDryRunOutput to separate files
2. Keep OutputHandler trait and NormalOutput in output.rs
3. Or merge into single output.rs with only 4 struct implementations

**Expected Results**:
- output.rs: ~4 functions (trait + create_output_handler)

### Phase 5: Documentation Updates
**Goal**: Update documentation to reflect new structure

1. Update docs/architecture.md (if exists, or create)
2. Update wiki documentation
3. Update docs/status.md with new structure

---

## Test Organization Strategy

### Shared Test Helpers (`tests/test_helpers.rs`)
```rust
// Common test utilities
pub fn create_temp_project(include_binary: bool) -> TempDir { ... }
pub fn create_workspace_project(members: &[&str]) -> TempDir { ... }
pub fn create_multi_component_project() -> TempDir { ... }
```

### Test File Naming Convention
```
tests/
├── test_helpers.rs         # Shared utilities (pub mod)
├── config_tests.rs         # Tests for config.rs
├── error_tests.rs          # Tests for error.rs
├── output_tests.rs         # Tests for output.rs
├── validator_tests.rs      # Tests for validation/
├── installer_tests.rs      # Tests for operations/installer.rs
├── uninstaller_tests.rs    # Tests for operations/uninstaller.rs
├── lister_tests.rs         # Tests for operations/lister.rs
└── setup_tests.rs          # Tests for operations/setup.rs
```

---

## Success Criteria

After refactoring, sw-checklist should show:

### All PASS
- Rust Edition: 2024
- All File LOC: < 500 lines
- All Function LOC: < 50 lines
- All Module Function Counts: ≤ 7 functions
- Crate Module Count: ≤ 7 modules

### Warnings Acceptable
- Function LOC > 25 lines (warnings OK)
- File LOC > 350 lines (warnings OK)
- Module Function Count > 4 (warnings OK)

### Test Coverage
- All existing tests continue to pass
- Tests organized in tests/ directory
- Shared test helpers reduce duplication

---

## Risk Mitigation

1. **Incremental Changes**: Each phase is independently testable
2. **Test-First**: Run tests after each change
3. **Version Control**: Commit after each successful phase
4. **Rollback Plan**: Each phase can be reverted independently

---

## Estimated Effort

| Phase | Description | Effort |
|-------|-------------|--------|
| 1 | Move tests to integration tests | Medium |
| 2 | Split validator into submodules | Low |
| 3 | Split operations into submodules | Low |
| 4 | Reduce output.rs function count | Low |
| 5 | Documentation updates | Low |

**Total**: ~2-3 hours of focused work
