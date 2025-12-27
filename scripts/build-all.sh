#!/bin/bash
# Build sw-install in release mode

set -e

cd "$(dirname "$0")/.."

cargo build --release --manifest-path components/sw-install-cli/Cargo.toml

echo "Build complete: components/sw-install-cli/target/release/sw-install"
