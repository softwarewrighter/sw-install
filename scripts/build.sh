#!/bin/bash
# Build all sw-install components in dependency order

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

echo "Building sw-install components..."

# Build in dependency order
cd "$REPO_ROOT/components/sw-install-core"
echo "Building sw-install-core..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-workspace"
echo "Building sw-install-workspace..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-validation"
echo "Building sw-install-validation..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-installer"
echo "Building sw-install-installer..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-manage"
echo "Building sw-install-manage..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-list"
echo "Building sw-install-list..."
cargo build --release

cd "$REPO_ROOT/components/sw-install-cli"
echo "Building sw-install-cli..."
cargo build --release

echo "Build complete!"
echo "Binary: $REPO_ROOT/components/sw-install-cli/target/release/sw-install"
