# Development Process

## sw-install

### Overview
This document outlines the development workflow, coding standards, and quality gates for the sw-install project.

### Development Methodology

#### Test-Driven Development (TDD)
All features must follow the Red-Green-Refactor cycle:

1. **Red**: Write a failing test
   - Write test first, before implementation
   - Test should fail initially
   - Verify test failure is for the right reason

2. **Green**: Make the test pass
   - Write minimal code to pass the test
   - Focus on functionality, not perfection
   - All tests must pass

3. **Refactor**: Improve the code
   - Clean up implementation
   - Remove duplication
   - Improve readability
   - All tests must still pass

#### Feature Development Workflow

1. **Plan**: Document the feature in docs/plan.md
2. **Design**: Update docs/design.md if architecture changes
3. **Test**: Write unit tests (Red phase)
4. **Implement**: Write code to pass tests (Green phase)
5. **Refactor**: Clean up code (Refactor phase)
6. **Validate**: Run pre-commit checks
7. **Document**: Update relevant docs
8. **Commit**: Commit with descriptive message

### Pre-Commit Quality Gates

Before committing ANY code, run the following checks in order:

#### 1. Format Check
```bash
cargo fmt --all -- --check
```
If this fails, format the code:
```bash
cargo fmt --all
```

#### 2. Clippy Lints
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
**CRITICAL**:
- Fix ALL clippy warnings
- DO NOT use #[allow(clippy::...)] to suppress warnings
- Fix the ROOT CAUSE, not the symptom
- Do not increase technical debt

Common fixes:
- Use `#[must_use]` where appropriate
- Implement `Display` instead of `ToString`
- Use `?` operator instead of `unwrap()`
- Remove unused code
- Fix naming conventions

#### 3. Build Check
```bash
cargo build --all-targets --all-features
```
- Must build without warnings
- Fix all warnings, do not ignore them

#### 4. Test Suite
```bash
cargo test --all-features
```
- All tests must pass
- No ignored tests in production code
- Aim for high code coverage

#### 5. Validate .gitignore
Ensure .gitignore includes:
```
/target
**/*.rs.bk
*.swp
*~
.DS_Store
Cargo.lock  # For binaries, include for libraries
```

Check for untracked files that should be ignored:
```bash
git status --porcelain
```

#### 6. Documentation Check
- Verify all docs/*.md files are UTF-8 (ASCII subset)
- No unprintable characters (tree symbols, etc.)
- Run documentation validation:
```bash
# Check for non-ASCII characters
find docs -name "*.md" -exec grep -P "[^\x00-\x7F]" {} + || echo "All docs are ASCII-clean"
```

- Update docs/status.md with current progress
- Ensure README.md reflects current functionality

### Pre-Commit Script

Create a pre-commit hook or use this manual checklist:

```bash
#!/bin/bash
# File: scripts/pre-commit-check.sh

set -e

echo "=== Pre-commit Quality Checks ==="

echo "[1/6] Formatting code..."
cargo fmt --all

echo "[2/6] Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "[3/6] Building..."
cargo build --all-targets --all-features

echo "[4/6] Running tests..."
cargo test --all-features

echo "[5/6] Checking .gitignore..."
if git status --porcelain | grep -E "^\?\?.*\.(swp|~|rs\.bk)$"; then
    echo "ERROR: Temporary files not in .gitignore"
    exit 1
fi

echo "[6/6] Validating documentation..."
if find docs README.md -name "*.md" -exec grep -P "[^\x00-\x7F]" {} + 2>/dev/null; then
    echo "ERROR: Non-ASCII characters found in documentation"
    exit 1
fi

echo ""
echo "All checks passed!"
```

### Code Style Guidelines

#### Rust Style
- Follow the official Rust Style Guide
- Use `cargo fmt` for automatic formatting
- Maximum line length: 100 characters
- Use meaningful variable names
- Prefer `?` operator over `unwrap()` or `expect()`

#### Error Handling
- Use `Result<T, InstallError>` for all fallible operations
- Define custom error types with `thiserror`
- Provide helpful error messages
- Include context in error propagation

#### Documentation
- Document all public APIs with /// doc comments
- Include examples in doc comments
- Document error conditions
- Keep docs synchronized with code

#### Testing
- One test per behavior
- Use descriptive test names: `test_validator_fails_when_project_missing`
- Use test fixtures for complex setup
- Prefer integration tests for end-to-end scenarios

### Git Workflow

#### Branch Strategy
- `main`: stable, release-ready code
- Feature branches: `feature/description`
- Bug fix branches: `fix/description`

#### Commit Messages
Format:
```
<type>: <short summary>

<detailed description if needed>

<issue references>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding or updating tests
- `refactor`: Code restructuring without behavior change
- `chore`: Build, tools, dependencies

Example:
```
feat: add dry-run mode for installation

Implement --dry-run flag that prints actions without executing them.
Useful for previewing installation without modifying the filesystem.

Related to #12
```

#### Commit Frequency
- Commit after each Red-Green-Refactor cycle
- Commit when all pre-commit checks pass
- Keep commits focused and atomic

### Code Review Checklist

Before submitting for review:
- [ ] All pre-commit checks pass
- [ ] Tests cover new functionality
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Error messages are clear and actionable
- [ ] No hardcoded paths or magic values
- [ ] Follows TDD methodology
- [ ] Code is readable and well-structured

### Continuous Improvement

#### Regular Tasks
- Review and update technical debt
- Refactor when patterns emerge
- Update documentation for clarity
- Add tests for edge cases
- Performance profiling for bottlenecks

#### Quality Metrics
Track these over time:
- Test coverage percentage
- Clippy warning count (should be 0)
- Documentation coverage
- Build time
- Test execution time

### Common Pitfalls to Avoid

1. **Skipping tests**: Never skip TDD process
2. **Suppressing warnings**: Fix root causes
3. **Hardcoding paths**: Use configuration
4. **Unclear errors**: Provide context and suggestions
5. **Inconsistent formatting**: Run cargo fmt
6. **Outdated docs**: Update with code changes
7. **Large commits**: Break into smaller pieces
8. **Ignoring edge cases**: Test boundary conditions

### Release Process

1. Ensure all tests pass
2. Update version in Cargo.toml
3. Update CHANGELOG.md
4. Update docs/status.md
5. Tag release: `git tag v0.1.0`
6. Build release binary: `cargo build --release`
7. Create GitHub release
8. Document known issues

### Emergency Fixes

For critical bugs:
1. Create fix branch from main
2. Write failing test demonstrating bug
3. Fix bug (make test pass)
4. Run all pre-commit checks
5. Fast-track review
6. Merge and release patch version
