#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ─── Handle optional logging argument ───────────────────────────────────────
LOGGING_ENABLED=false

for arg in "$@"; do
  if [[ "$arg" == "--log" ]]; then
    LOGGING_ENABLED=true
  fi
done

if $LOGGING_ENABLED; then
  FRONTEND_LOG="$SCRIPT_DIR/frontend.log"
  VITE_LOG="$SCRIPT_DIR/vite.log"
  RUST_LOG="$SCRIPT_DIR/cargo.log"
  PYTHON_LOG="$SCRIPT_DIR/mock_sender.log"
else
  FRONTEND_LOG="/dev/null"
  VITE_LOG="/dev/null"
  RUST_LOG="/dev/null"
  PYTHON_LOG="/dev/null"
fi

# ─── 1) Build frontend ─────────────────────────────────────────────────────
cd "$SCRIPT_DIR/frontend"
echo "🔨 Building frontend…"
npm run build > "$FRONTEND_LOG" 2>&1

echo "🌐 Starting Vite dev server in background (npm run dev)…"
npm run dev > "$VITE_LOG" 2>&1 &
VITE_PID=$!

# ─── 2) Launch Rust server ─────────────────────────────────────────────────
cd "$SCRIPT_DIR"
echo "🦀 Starting Rust server… (logging to $RUST_LOG)"
cargo run -- --no-can > "$RUST_LOG" 2>&1 &
RUST_PID=$!

sleep 5
# ─── 3) Wait for backend to be ready (port 3001) ───────────────────────────
echo "⏳ Waiting for WebSocket server on port 3001…"
while ! echo > /dev/tcp/localhost/3001 2>/dev/null; do
  sleep 1
done
echo "✅ WebSocket server is ready."

# ─── 4) Start Python mock (after backend is ready) ─────────────────────────
echo "🐍 Running mock_sender.py…"
python mock_sender.py > "$PYTHON_LOG" 2>&1 &
PYTHON_PID=$!

# ─── 5) Cleanup trap ───────────────────────────────────────────────────────
cleanup() {
  echo
  echo "🧹 Cleaning up…"
  kill "$VITE_PID" "$RUST_PID" "$PYTHON_PID" 2>/dev/null || true
  wait "$VITE_PID" "$RUST_PID" "$PYTHON_PID" 2>/dev/null || true
}
trap cleanup SIGINT SIGTERM EXIT

# ─── 6) Hold script open ───────────────────────────────────────────────────
echo "🚀 Dev environment started."
echo "Frontend: http://localhost:3000"
echo "Press Ctrl+C to stop."
wait
