#!/bin/bash
# Run sw-install from the release build, passing all arguments

set -e

cd "$(dirname "$0")/.."

# Build if needed
if [ ! -f target/release/sw-install ]; then
    cargo build --release
fi

exec ./target/release/sw-install "$@"
