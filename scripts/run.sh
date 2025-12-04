#!/bin/bash
# Run sw-install from the release build, passing all arguments

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_ROOT/components/sw-install-cli/target/release/sw-install"

# Build if needed
if [ ! -f "$BINARY" ]; then
    "$SCRIPT_DIR/build.sh"
fi

exec "$BINARY" "$@"
