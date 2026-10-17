#!/usr/bin/env bash
# Builds a macOS .app bundle for hangboard_timer under ./target
set -euo pipefail

cd "$(dirname "$0")"

APP_NAME="hangboard_timer"
BUNDLE_ID="com.robson021.hangboard-timer"
VERSION="$(sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
APP_DIR="target/$APP_NAME.app"

if [[ "$(uname)" != "Darwin" ]]; then
  echo "This script must be run on macOS" >&2
  exit 1
fi

cargo build --release

rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources"
cp "target/release/$APP_NAME" "$APP_DIR/Contents/MacOS/$APP_NAME-bin"
cp -R assets "$APP_DIR/Contents/Resources/assets"

cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>$APP_NAME</string>
  <key>CFBundleDisplayName</key><string>$APP_NAME</string>
  <key>CFBundleExecutable</key><string>$APP_NAME</string>
  <key>CFBundleIdentifier</key><string>$BUNDLE_ID</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST

# Sounds are loaded via the relative path "assets/sound/...", so run from Resources.
cat > "$APP_DIR/Contents/MacOS/$APP_NAME" <<LAUNCHER
#!/bin/bash
DIR="\$(cd "\$(dirname "\$0")" && pwd)"
cd "\$DIR/../Resources"
exec "\$DIR/$APP_NAME-bin" "\$@"
LAUNCHER
chmod +x "$APP_DIR/Contents/MacOS/$APP_NAME"

# Ad-hoc sign so the app runs on Apple Silicon
codesign --force --deep --sign - "$APP_DIR"

echo "Created: $APP_DIR"
