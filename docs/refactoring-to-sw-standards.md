# Refactoring to Software Wrighter Standards

This document captures the architectural patterns, design principles, and refactoring strategies learned from analyzing well-structured Rust projects that pass sw-checklist validation. It serves as a general-purpose guide for refactoring any Rust CLI project to meet Software Wrighter coding standards.

---

## Table of Contents

1. [Overview](#overview)
2. [Core Design Principles](#core-design-principles)
3. [Module Organization Strategies](#module-organization-strategies)
4. [Test Organization Patterns](#test-organization-patterns)
5. [Function and File Size Constraints](#function-and-file-size-constraints)
6. [Documentation Patterns](#documentation-patterns)
7. [Refactoring Workflow](#refactoring-workflow)
8. [Common Refactoring Patterns](#common-refactoring-patterns)
9. [sw-checklist Compliance Checklist](#sw-checklist-compliance-checklist)

---

## Overview

### What is sw-checklist?

sw-checklist is a validation tool that enforces coding standards designed to produce small, documented, tested, maintainable units of code. It validates:

- **Rust edition** (must be 2024)
- **Function line counts** (warning >25, fail >50)
- **File line counts** (warning >350, fail >500)
- **Module function counts** (warning >4, fail >7)
- **Crate module counts** (max 7 modules)
- **CLI standards** (help text, version info, AI agent instructions)

### Why These Constraints?

The constraints are based on **Miller's Law (7±2 rule)** - the cognitive limit on the number of items a person can hold in working memory. By keeping modules small and focused:

- Code is easier to understand
- Functions fit on a single screen
- Modules have clear single responsibilities
- Testing becomes straightforward
- Refactoring is safer and easier

---

## Core Design Principles

### 1. Single Responsibility Principle

Each module, struct, and function should have one clear purpose:

```
GOOD:
  parse.rs    → parsing only
  validate.rs → validation only
  format.rs   → serialization only

BAD:
  utils.rs    → mixed parsing, validation, formatting, helpers
```

### 2. Separation of Tests from Implementation

Tests should live in a separate `tests/` directory, not embedded in source files:

```
GOOD:
  src/validator.rs       (implementation only)
  tests/validator_tests.rs (all tests)

BAD:
  src/validator.rs       (implementation + #[cfg(test)] mod tests)
```

**Rationale**: Embedded tests count toward module function limits and inflate file sizes.

### 3. lib.rs Contains Only Re-exports

The library root should only contain `pub mod` and `pub use` statements:

```rust
// GOOD: lib.rs
pub mod config;
pub mod error;
pub mod operations;

pub use config::Config;
pub use error::{Error, Result};
pub use operations::{Installer, Lister};
```

```rust
// BAD: lib.rs with function implementations
pub mod config;

pub fn helper_function() { ... }  // Should be in a module
```

### 4. Pure Functions Separated from I/O

Separate pure logic from side effects:

```rust
// GOOD: Pure function (easy to test)
fn calculate_time_ago(now: SystemTime, then: SystemTime) -> String { ... }

// Separate I/O function
fn print_binary_list(binaries: &[BinaryInfo]) { ... }
```

### 5. Data-First Design

Define core data types with minimal dependencies, then build logic on top:

```
Layer 1: Data types (structs, enums) - no logic, just serde
Layer 2: Pure functions operating on data types
Layer 3: I/O and side effects
Layer 4: CLI orchestration
```

### 6. Explicit Dependency Layering

Lower layers should never depend on higher layers:

```
config.rs (lowest - no internal deps)
    ↓
error.rs (depends on nothing)
    ↓
validator.rs (depends on config, error)
    ↓
installer.rs (depends on config, error, validator)
    ↓
main.rs (depends on everything)
```

### 7. Refactor Upward, Never Downward

When faced with constraint violations (too many functions, too many modules), always prefer **splitting upward** over **merging downward**.

**The Refactoring Direction Principle:**

```
Functions → Modules → Crates → Components
     ↑          ↑         ↑
   SPLIT      SPLIT     SPLIT
   (good)     (good)    (good)

Functions ← Modules ← Crates ← Components
     ↓          ↓         ↓
   MERGE      MERGE     MERGE
   (avoid)    (avoid)   (avoid)
```

**Why Splitting is Preferred:**

1. **Maintains Separation of Concerns**: Splitting creates clear boundaries between concepts
2. **Enables Future Growth**: Split code is easier to extend without violating constraints
3. **Improves Testability**: Smaller, focused units are easier to test in isolation
4. **Reduces Cognitive Load**: Each unit remains within Miller's Law limits
5. **Creates Reusable Components**: Split code can be shared across projects

**Why Merging is Problematic:**

1. **Delays Inevitable Refactoring**: Merged code will eventually need to be split anyway
2. **Creates Hidden Dependencies**: Merged modules often have tangled internal dependencies
3. **Increases Cognitive Load**: Combined modules require understanding more context at once
4. **Reduces Clarity**: The purpose of a merged module becomes less clear
5. **Makes Testing Harder**: Merged code has more internal state to manage

**Example - Wrong Approach (Merging Down):**

```rust
// WRONG: Merging error.rs and config.rs into lib.rs to reduce module count
// lib.rs now has 10+ functions and multiple responsibilities

pub enum InstallError { ... }
pub struct InstallConfig { ... }
impl InstallConfig { ... }  // 5 functions
// Plus re-exports
```

**Example - Right Approach (Splitting Up):**

```rust
// RIGHT: Split validator.rs (14 functions) into submodules
// Creates: src/validation/mod.rs + project.rs + binary.rs + freshness.rs

// src/validation/mod.rs (3 functions - orchestration only)
mod project;
mod binary;
mod freshness;

pub use project::ProjectType;
pub struct Validator { ... }

impl Validator {
    pub fn validate(&self) -> Result<ValidationResult> {
        let project_type = project::detect(&self.config.project_path)?;
        let binary_name = binary::extract(&project_type)?;
        freshness::check(&binary_path)?;
        // ...
    }
}

// src/validation/project.rs (4 functions)
// src/validation/binary.rs (4 functions)
// src/validation/freshness.rs (3 functions)
```

**When You Think You Need to Merge, Instead:**

1. **Too many modules?** → Create a subdirectory with mod.rs, move related modules into it
2. **Too many functions in a module?** → Split into submodules by responsibility
3. **Too many lines in a file?** → Extract types, helpers, or tests into separate files
4. **lib.rs getting complex?** → Keep it as pure re-exports, move logic to modules

**The Constraint Escalation Path:**

When a constraint is violated, consider these options in order:

1. **Extract tests** to `tests/` directory (immediate win, always do this first)
2. **Split functions** into helpers (reduces function line count)
3. **Create submodules** within the same directory (reduces module function count)
4. **Create a new crate** in the workspace (for larger extractions)
5. **Create a new component** (for major architectural boundaries)

Never consider merging until you've exhausted all splitting options.

---

## Module Organization Strategies

### Flat vs. Hierarchical Structure

**Flat Structure** (≤7 modules):
```
src/
├── lib.rs
├── config.rs
├── error.rs
├── validator.rs
├── installer.rs
└── main.rs
```

**Hierarchical Structure** (>7 logical modules):
```
src/
├── lib.rs
├── config.rs
├── error.rs
├── validation/
│   ├── mod.rs
│   ├── validator.rs
│   └── project_type.rs
├── operations/
│   ├── mod.rs
│   ├── installer.rs
│   └── lister.rs
└── main.rs
```

### When to Use Submodules

Use submodules when:
- A single module exceeds 7 functions
- Related functionality can be logically grouped
- You need to reduce top-level module count

### Submodule mod.rs Pattern

```rust
// operations/mod.rs
mod installer;
mod lister;
mod setup;
mod uninstaller;

pub use installer::Installer;
pub use lister::{Lister, SortOrder};
pub use setup::Setup;
pub use uninstaller::Uninstaller;
```

### Counting Modules for sw-checklist

sw-checklist counts `.rs` files as modules. To minimize count:
- Use submodules with `mod.rs` (counts as 1 module per directory)
- Merge small related modules
- Keep `lib.rs` as re-exports only

---

## Test Organization Patterns

### Directory Structure

```
tests/
├── test_helpers.rs      # Shared utilities (optional)
├── config_tests.rs      # Tests for src/config.rs
├── validator_tests.rs   # Tests for src/validator.rs
├── installer_tests.rs   # Tests for src/installer.rs
└── integration_tests.rs # End-to-end tests (optional)
```

### Shared Test Helpers

Create reusable test utilities:

```rust
// tests/test_helpers.rs
use std::path::Path;
use tempfile::TempDir;

pub fn create_temp_project(include_binary: bool) -> TempDir {
    let temp = TempDir::new().unwrap();
    // Setup project structure...
    temp
}

pub fn create_workspace_project(members: &[&str]) -> TempDir {
    let temp = TempDir::new().unwrap();
    // Setup workspace structure...
    temp
}
```

### Using Test Helpers

```rust
// tests/validator_tests.rs
mod test_helpers;

use test_helpers::create_temp_project;
use sw_install::{InstallConfig, Validator};

#[test]
fn test_validate_succeeds() {
    let temp = create_temp_project(true);
    let config = InstallConfig::new(temp.path().to_path_buf(), ...);
    // Test logic...
}
```

### Test Naming Convention

```
tests/
├── {module}_tests.rs    # Unit/integration tests for a module
├── test_helpers.rs      # Shared utilities
└── common/mod.rs        # Alternative for shared code
```

---

## Function and File Size Constraints

### Function Line Limits

| Threshold | Status | Action |
|-----------|--------|--------|
| ≤25 lines | OK | No action needed |
| 26-50 lines | WARNING | Consider splitting |
| >50 lines | FAIL | Must split |

### Strategies to Reduce Function Size

**1. Extract Helper Functions**

```rust
// BEFORE: 60-line function
fn process_data(data: &Data) -> Result<Output> {
    // validation (15 lines)
    // transformation (20 lines)
    // formatting (15 lines)
    // output (10 lines)
}

// AFTER: 4 smaller functions
fn process_data(data: &Data) -> Result<Output> {
    validate_data(data)?;
    let transformed = transform_data(data)?;
    let formatted = format_output(&transformed);
    Ok(formatted)
}

fn validate_data(data: &Data) -> Result<()> { ... }
fn transform_data(data: &Data) -> Result<Transformed> { ... }
fn format_output(data: &Transformed) -> Output { ... }
```

**2. Use Early Returns**

```rust
// BEFORE: Nested conditionals
fn check_project(path: &Path) -> Result<ProjectType> {
    if path.exists() {
        if path.is_dir() {
            let cargo = path.join("Cargo.toml");
            if cargo.exists() {
                // 20 more lines...
            }
        }
    }
}

// AFTER: Early returns
fn check_project(path: &Path) -> Result<ProjectType> {
    if !path.exists() {
        return Err(Error::NotFound);
    }
    if !path.is_dir() {
        return Err(Error::NotDirectory);
    }
    let cargo = path.join("Cargo.toml");
    if !cargo.exists() {
        return Err(Error::NoCargoToml);
    }
    // Continue with main logic...
}
```

**3. Use Iterator Chains**

```rust
// BEFORE: Explicit loops
fn find_binaries(dir: &Path) -> Vec<String> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name() {
                result.push(name.to_string_lossy().to_string());
            }
        }
    }
    result
}

// AFTER: Iterator chain
fn find_binaries(dir: &Path) -> Vec<String> {
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().to_str().map(String::from))
        .collect()
}
```

### File Line Limits

| Threshold | Status | Action |
|-----------|--------|--------|
| ≤350 lines | OK | No action needed |
| 351-500 lines | WARNING | Consider splitting |
| >500 lines | FAIL | Must split |

### Strategies to Reduce File Size

1. **Move tests to `tests/` directory** - Often reduces file by 30-50%
2. **Split into submodules** - Group related functions
3. **Extract data types** - Move structs/enums to separate file
4. **Remove dead code** - Run `cargo clippy` to identify

---

## Documentation Patterns

### Required Documentation Files

```
docs/
├── status.md        # Current project status
├── plan.md          # Development roadmap
├── architecture.md  # System design (optional but recommended)
├── design.md        # Detailed design decisions (optional)
└── learnings.md     # Lessons learned (optional)
```

### Module-Level Documentation

```rust
//! Binary installer for softwarewrighter CLI projects.
//!
//! This module provides the core installation functionality,
//! copying compiled binaries to the installation directory.
//!
//! # Example
//!
//! ```
//! use sw_install::{InstallConfig, Installer};
//!
//! let config = InstallConfig::new(...);
//! let installer = Installer::new(&config);
//! installer.install()?;
//! ```
```

### Function Documentation

```rust
/// Validates the project structure and returns binary information.
///
/// # Arguments
///
/// * `config` - Installation configuration
///
/// # Returns
///
/// * `Ok(ValidationResult)` - Binary name and source path
/// * `Err(InstallError)` - If validation fails
///
/// # Errors
///
/// Returns an error if:
/// - Project path doesn't exist
/// - Cargo.toml is missing
/// - Binary is not compiled
pub fn validate(&self) -> Result<ValidationResult> { ... }
```

---

## Refactoring Workflow

### Phase 1: Assessment

1. Run `sw-checklist` to identify all failures and warnings
2. Count functions in each module
3. Identify largest files and functions
4. Plan refactoring order (start with biggest wins)

### Phase 2: Move Tests

1. Create `tests/` directory structure
2. Create `tests/test_helpers.rs` if needed
3. Move tests from each module to `tests/{module}_tests.rs`
4. Remove `#[cfg(test)] mod tests` blocks from source
5. Run `cargo test` to verify

### Phase 3: Split Large Modules

1. Identify modules with >7 functions
2. Group related functions
3. Create submodule directory with `mod.rs`
4. Move function groups to separate files
5. Update `mod.rs` with re-exports
6. Update parent module imports
7. Run `cargo test` to verify

### Phase 4: Split Large Functions

1. Identify functions >50 lines
2. Extract helper functions
3. Use early returns to flatten logic
4. Run `cargo test` to verify

### Phase 5: Verify and Document

1. Run `sw-checklist` - all checks should pass
2. Run `cargo clippy` - no warnings
3. Run `cargo fmt` - consistent formatting
4. Update documentation to reflect new structure

---

## Common Refactoring Patterns

### Pattern 1: Test Extraction

**Before:**
```rust
// src/validator.rs (400 lines)
pub struct Validator { ... }

impl Validator {
    pub fn validate(&self) -> Result<()> { ... }
    fn helper(&self) { ... }
}

#[cfg(test)]
mod tests {
    // 200 lines of tests
}
```

**After:**
```rust
// src/validator.rs (200 lines)
pub struct Validator { ... }

impl Validator {
    pub fn validate(&self) -> Result<()> { ... }
    fn helper(&self) { ... }
}

// tests/validator_tests.rs (200 lines)
use my_crate::Validator;

#[test]
fn test_validate() { ... }
```

### Pattern 2: Submodule Extraction

**Before:**
```rust
// src/validator.rs (12 functions)
pub struct Validator { ... }
enum ProjectType { ... }

impl Validator {
    pub fn validate(&self) { ... }
    fn detect_project_type(&self) { ... }
    fn find_workspace_binaries(&self) { ... }
    // ... 9 more functions
}
```

**After:**
```rust
// src/validation/mod.rs
mod validator;
mod project_type;
mod workspace;

pub use validator::Validator;

// src/validation/validator.rs (4 functions)
use super::project_type::ProjectType;
use super::workspace;

pub struct Validator { ... }

impl Validator {
    pub fn validate(&self) { ... }
    // 3 more core functions
}

// src/validation/project_type.rs (3 functions)
pub enum ProjectType { ... }

pub fn detect(path: &Path) -> Result<ProjectType> { ... }

// src/validation/workspace.rs (3 functions)
pub fn find_binaries(path: &Path) -> Result<Vec<String>> { ... }
```

### Pattern 3: Output Handler Consolidation

**Before:**
```rust
// src/output.rs (22 functions - 4 structs × 4 methods + factory + trait)
pub trait OutputHandler { ... }
pub struct NormalOutput;
pub struct VerboseOutput;
pub struct DryRunOutput;
pub struct VerboseDryRunOutput;

impl OutputHandler for NormalOutput { ... }  // 4 methods
impl OutputHandler for VerboseOutput { ... }  // 4 methods
impl OutputHandler for DryRunOutput { ... }  // 4 methods
impl OutputHandler for VerboseDryRunOutput { ... }  // 4 methods

pub fn create_output_handler(...) { ... }
```

**After:**
```rust
// src/output.rs (5 functions)
pub trait OutputHandler {
    fn info(&self, msg: &str);
    fn step(&self, msg: &str);
    fn success(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Output {
    verbose: bool,
    dry_run: bool,
}

impl OutputHandler for Output {
    fn info(&self, msg: &str) {
        if self.verbose { println!("{}", msg); }
    }
    // ... other methods
}

pub fn create_output_handler(verbose: bool, dry_run: bool) -> Output {
    Output { verbose, dry_run }
}
```

---

## sw-checklist Compliance Checklist

### Before Submitting Code

- [ ] `cargo fmt` - Code is formatted
- [ ] `cargo clippy` - No warnings
- [ ] `cargo test` - All tests pass
- [ ] `sw-checklist` - All checks pass or only warnings

### Module Checklist

- [ ] ≤7 functions per module (including impl methods)
- [ ] ≤500 lines per file
- [ ] No tests in source files (move to `tests/`)
- [ ] `lib.rs` contains only re-exports

### Function Checklist

- [ ] ≤50 lines per function
- [ ] Single responsibility
- [ ] Early returns for error cases
- [ ] Pure functions where possible

### Crate Checklist

- [ ] ≤7 top-level modules
- [ ] Rust 2024 edition
- [ ] Clear dependency hierarchy
- [ ] Comprehensive documentation

### CLI Checklist (for CLI tools)

- [ ] `--help` longer than `-h`
- [ ] Version info includes: Copyright, License, Repository
- [ ] Version info includes: Build Host, Commit, Timestamp
- [ ] AI Agent Instructions section in extended help

---

## References

### Projects Following These Standards

- **sw-checklist**: The validation tool itself, demonstrating modular check architecture
- **alltalk-client-rs**: Multi-component workspace with strict size constraints

### Key Documentation

- sw-checklist `docs/design.md` - Design decisions and rationale
- sw-checklist `docs/architecture.md` - System architecture
- alltalk-client-rs `docs/design.md` - Code constraints and patterns

### Rust Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book - Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Rust Book - Separating Modules](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
