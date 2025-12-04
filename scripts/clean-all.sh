#!/bin/bash
# Clean all target directories in multi-component structure

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COMPONENTS_DIR="$PROJECT_ROOT/components"

echo "Cleaning all target directories..."

for dir in "$COMPONENTS_DIR"/*/target; do
    if [ -d "$dir" ]; then
        echo "  Removing $dir"
        rm -rf "$dir"
    fi
done

echo "Clean complete."
