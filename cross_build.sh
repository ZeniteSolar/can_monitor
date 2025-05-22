#!/usr/bin/env bash

set -e  # Exit on error

# Arguments
REMOTE_HOST=$1
if [ -z "$REMOTE_HOST" ]; then
  echo "❌ ERROR: Please provide the target IP or hostname."
  echo "Usage: $0 <ip-address-or-hostname>"
  exit 1
fi

# Config
TARGET=armv7-unknown-linux-gnueabihf
BINARY_NAME=can_parser_rs
BUILD_DIR=$PWD/target/$TARGET/release
REMOTE_USER=zenite
REMOTE_PATH="~/$BINARY_NAME"

# Build
echo "📦 Building for target: $TARGET..."
cross build --release --target=$TARGET
echo "✅ Build complete. Binary at: $BUILD_DIR/$BINARY_NAME"

# Deploy
echo "📤 Deploying to $REMOTE_USER@$REMOTE_HOST..."
scp "$BUILD_DIR/$BINARY_NAME" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"

# Restart service
echo "🚀 Restarting service on Pi..."
if ssh $REMOTE_USER@$REMOTE_HOST 'sudo systemctl restart simple_can_monitor.service'; then
  echo "✅ Service restarted successfully."
else
  echo "❌ Failed to restart service."
  exit 1
fi

echo "✅ Done. Deployed to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"
