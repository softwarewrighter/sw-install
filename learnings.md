# Learnings

This document captures key insights and lessons learned during development of sw-install.

## Test-Driven Development (TDD)

### Adding List Feature (2025-11-16)

**Context**: Added feature to list installed binaries in ~/.local/softwarewrighter/bin and verified uninstall functionality.

**Approach**:
1. Created comprehensive TDD tests first before implementation
2. Created `src/lister.rs` module with 7 test cases covering:
   - Empty directory
   - Single binary
   - Multiple binaries (with sorting verification)
   - Ignoring directories (only listing files)
   - Non-existent installation directory
   - Test directory usage
   - Alphabetical sorting

**Tests Created**:
- `test_list_no_binaries`: Verifies handling of empty installation directory
- `test_list_single_binary`: Tests single binary listing
- `test_list_multiple_binaries`: Tests multiple binaries with proper count
- `test_list_ignores_directories`: Ensures subdirectories are excluded
- `test_list_fails_when_dir_does_not_exist`: Validates error handling
- `test_destination_dir_with_test_dir`: Tests test mode functionality
- `test_list_sorted_output`: Verifies alphabetical sorting

**Key Lessons**:
1. **Tests Drive Design**: Writing tests first helped identify the need for:
   - Proper error handling when installation directory doesn't exist
   - Filtering out directories from the list
   - Sorted output for consistent user experience
   - Test mode support via `test_dir` parameter

2. **Comprehensive Coverage**: TDD approach ensured edge cases were considered:
   - Empty directories
   - Non-existent directories
   - Mixed content (files and directories)
   - Proper sorting

3. **Integration**: Added the `Lister` module to:
   - `src/lib.rs` (module declaration and public export)
   - `src/main.rs` (CLI integration with `--list` flag)
   - Extended help text and examples

**Implementation Details**:
- Created `Lister` struct following existing patterns (`Uninstaller`, `Installer`)
- Used `OutputHandler` trait for consistent messaging
- Followed serial test pattern with `serial_test` crate
- Used `tempfile` for isolated test environments

**Results**:
- All 48 tests pass (including 7 new lister tests)
- Clean build with no warnings
- Feature integrated into CLI with proper help documentation

**Best Practices Reinforced**:
1. Write tests before implementation
2. Test edge cases and error conditions
3. Use temporary directories for file system tests
4. Follow existing code patterns and conventions
5. Integrate with existing infrastructure (OutputHandler, error types)

## Future Considerations

- Consider adding integration tests for end-to-end CLI workflows
- Could add JSON output format for programmatic consumption
- Might want to show installation metadata (date, version, source)
