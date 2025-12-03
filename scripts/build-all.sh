#!/bin/bash
# Build sw-install in release mode

set -e

cd "$(dirname "$0")/.."

cargo build --release

echo "Build complete: target/release/sw-install"
