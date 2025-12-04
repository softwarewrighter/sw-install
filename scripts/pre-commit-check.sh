#!/bin/bash
# Copyright (c) 2025 Michael A Wright
# Pre-commit quality checks for sw-install (multi-component)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$PROJECT_ROOT/components"

# Components in dependency order
COMPONENTS=(
    "sw-install-core"
    "sw-install-workspace"
    "sw-install-validation"
    "sw-install-installer"
    "sw-install-manage"
    "sw-install-list"
    "sw-install-cli"
)

echo "=== Pre-commit Quality Checks ==="
echo ""

echo "[1/6] Formatting code..."
for comp in "${COMPONENTS[@]}"; do
    cargo fmt --manifest-path "$COMPONENTS_DIR/$comp/Cargo.toml"
done
echo "  ✓ Code formatted"
echo ""

echo "[2/6] Running clippy..."
for comp in "${COMPONENTS[@]}"; do
    cargo clippy --manifest-path "$COMPONENTS_DIR/$comp/Cargo.toml" --all-targets -- -D warnings
done
echo "  ✓ No clippy warnings"
echo ""

echo "[3/6] Building..."
for comp in "${COMPONENTS[@]}"; do
    cargo build --manifest-path "$COMPONENTS_DIR/$comp/Cargo.toml" --release
done
echo "  ✓ Build successful"
echo ""

echo "[4/6] Running tests..."
cargo test --manifest-path "$COMPONENTS_DIR/sw-install-cli/Cargo.toml"
echo "  ✓ All tests passed"
echo ""

echo "[5/6] Checking .gitignore..."
if git -C "$PROJECT_ROOT" status --porcelain | grep -E "^\?\?.*\.(swp|~|rs\.bk)$"; then
    echo "  ERROR: Temporary files not in .gitignore"
    exit 1
fi
echo "  ✓ No temporary files found"
echo ""

echo "[6/6] Validating documentation..."
if find "$PROJECT_ROOT/docs" "$PROJECT_ROOT/README.md" -name "*.md" -exec grep -P "[^\x00-\x7F]" {} + 2>/dev/null; then
    echo "  ERROR: Non-ASCII characters found in documentation"
    exit 1
fi
echo "  ✓ All documentation is ASCII-clean"
echo ""

echo "========================================="
echo "All checks passed! Ready to commit."
echo "========================================="
