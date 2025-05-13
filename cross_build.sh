#!/usr/bin/env bash

set -e  # Exit on error

TARGET=armv7-unknown-linux-gnueabihf
BINARY_NAME=can_parser_rs
BUILD_DIR=$PWD/target/$TARGET/release
REMOTE_USER=zenite
REMOTE_HOST=192.168.172.59
REMOTE_PATH="~/$BINARY_NAME"

echo "Building for target: $TARGET..."
cross build --release --target=$TARGET

echo "Build complete. Binary at: $BUILD_DIR/$BINARY_NAME"

echo "Deploying to $REMOTE_USER@$REMOTE_HOST..."
scp "$BUILD_DIR/$BINARY_NAME" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"

echo "🚀 Restarting service on Pi..."
if ssh $REMOTE_USER@$REMOTE_HOST 'sudo systemctl restart simple_can_monitor.service'; then
  echo "✅ Service restarted successfully."
else
  echo "❌ Failed to restart service."
  exit 1
fi

echo "✅ Done. Deployed to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH