#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_DIR/build/pilot_0.1.0-1"

echo "=== Building Pilot deb package ==="

rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/opt/pilot/daemon"
mkdir -p "$BUILD_DIR/usr/bin"

echo "--- Building Tauri frontend ---"
cd "$PROJECT_DIR/tauri-app/ui"
npm ci
npm run build

echo "--- Building Tauri binary ---"
cd "$PROJECT_DIR/tauri-app/src-tauri"
cargo build --release

echo "--- Assembling package ---"

# Tauri binary
cp "$PROJECT_DIR/tauri-app/src-tauri/target/release/pilot" "$BUILD_DIR/usr/bin/pilot"
chmod 755 "$BUILD_DIR/usr/bin/pilot"

# Python daemon source (installed into venv by postinst)
cp -r "$PROJECT_DIR/daemon/"* "$BUILD_DIR/opt/pilot/daemon/"

# Staging area for files that postinst copies into system directories.
# This keeps the deb package self-contained — postinst reads from /opt/pilot/.
cp "$SCRIPT_DIR/pilot-daemon.service" "$BUILD_DIR/opt/pilot/pilot-daemon.service"
cp "$SCRIPT_DIR/pilot.desktop"        "$BUILD_DIR/opt/pilot/pilot.desktop"
cp "$SCRIPT_DIR/com.pilot.policy"     "$BUILD_DIR/opt/pilot/com.pilot.policy"

# DEBIAN control scripts
cp "$SCRIPT_DIR/debian/control"  "$BUILD_DIR/DEBIAN/control"
cp "$SCRIPT_DIR/debian/postinst" "$BUILD_DIR/DEBIAN/postinst"
cp "$SCRIPT_DIR/debian/prerm"    "$BUILD_DIR/DEBIAN/prerm"
chmod 755 "$BUILD_DIR/DEBIAN/postinst"
chmod 755 "$BUILD_DIR/DEBIAN/prerm"

echo "--- Building .deb ---"
dpkg-deb --build "$BUILD_DIR"

echo "=== Package built: $BUILD_DIR.deb ==="
