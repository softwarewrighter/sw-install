#!/bin/bash
# Copyright (c) 2025 Michael A Wright
# Pre-commit quality checks for sw-install

set -e

echo "=== Pre-commit Quality Checks ==="
echo ""

echo "[1/6] Formatting code..."
cargo fmt --all
echo "  ✓ Code formatted"
echo ""

echo "[2/6] Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
echo "  ✓ No clippy warnings"
echo ""

echo "[3/6] Building..."
cargo build --all-targets --all-features
echo "  ✓ Build successful"
echo ""

echo "[4/6] Running tests..."
cargo test --all-features
echo "  ✓ All tests passed"
echo ""

echo "[5/6] Checking .gitignore..."
if git status --porcelain | grep -E "^\?\?.*\.(swp|~|rs\.bk)$"; then
    echo "  ERROR: Temporary files not in .gitignore"
    exit 1
fi
echo "  ✓ No temporary files found"
echo ""

echo "[6/6] Validating documentation..."
if find docs README.md -name "*.md" -exec grep -P "[^\x00-\x7F]" {} + 2>/dev/null; then
    echo "  ERROR: Non-ASCII characters found in documentation"
    exit 1
fi
echo "  ✓ All documentation is ASCII-clean"
echo ""

echo "========================================="
echo "All checks passed! Ready to commit."
echo "========================================="
