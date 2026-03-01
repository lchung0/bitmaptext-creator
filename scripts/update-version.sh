#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CARGO_TOML="$PROJECT_ROOT/Cargo.toml"
LOCALES_ZH="$PROJECT_ROOT/docs/locales/zh.js"
LOCALES_EN="$PROJECT_ROOT/docs/locales/en.js"

if [ ! -f "$CARGO_TOML" ]; then
    echo "Error: Cargo.toml not found at $CARGO_TOML"
    exit 1
fi

VERSION=$(grep '^version =' "$CARGO_TOML" | sed -E 's/version = "([^"]+)"/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Could not extract version from Cargo.toml"
    exit 1
fi

echo "Found version: $VERSION"

if [ -f "$LOCALES_ZH" ]; then
    sed -i.bak -E "s/badge: '[^']+'/badge: 'v${VERSION} 现已发布'/" "$LOCALES_ZH"
    rm -f "$LOCALES_ZH.bak"
    echo "Updated $(basename "$LOCALES_ZH")"
fi

if [ -f "$LOCALES_EN" ]; then
    sed -i.bak -E "s/badge: '[^']+'/badge: 'v${VERSION} Now Available'/" "$LOCALES_EN"
    rm -f "$LOCALES_EN.bak"
    echo "Updated $(basename "$LOCALES_EN")"
fi

echo "All files updated successfully!"
