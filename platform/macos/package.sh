#!/usr/bin/env bash
# Packs the desktop runtime as a universal (arm64 + x86_64) Ethornell.app.
# Opening the app asks for the game folder; `Ethornell.app/Contents/MacOS/
# ethornell <folder>` plays one directly.
#
#   platform/macos/package.sh   # -> target/macos/Ethornell.app, Ethornell-macos.zip
set -euo pipefail

MACOS_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$MACOS_DIR/../.." && pwd)"
OUT="$ROOT/target/macos"
APP="$OUT/Ethornell.app"
export MACOSX_DEPLOYMENT_TARGET="${MACOSX_DEPLOYMENT_TARGET:-11.0}"

rustup target add aarch64-apple-darwin x86_64-apple-darwin >/dev/null
cd "$ROOT"
for target in aarch64-apple-darwin x86_64-apple-darwin; do
  cargo build --release --locked -p ethornell-cli --bin ethornell --target "$target"
done

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
lipo -create \
  target/aarch64-apple-darwin/release/ethornell \
  target/x86_64-apple-darwin/release/ethornell \
  -output "$APP/Contents/MacOS/ethornell"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/ethornell-cli/Cargo.toml | head -n 1)"
sed "s/@VERSION@/$VERSION/g" "$MACOS_DIR/Info.plist" > "$APP/Contents/Info.plist"
# Ad-hoc signature: enough to run on Apple Silicon. Without a Developer ID,
# the first launch needs right-click > Open (or System Settings > Privacy &
# Security > Open Anyway).
codesign --force --deep --sign - "$APP"

rm -f "$OUT/Ethornell-macos.zip"
ditto -c -k --keepParent "$APP" "$OUT/Ethornell-macos.zip"
echo "$APP"
echo "$OUT/Ethornell-macos.zip"
