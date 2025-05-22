#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ─── 1) Build frontend ─────────────────────────────────────────────────────
cd "$SCRIPT_DIR/frontend"
echo "🔨 Building frontend…"
npm run build

echo "🌐 Starting Vite dev server in background (npm run dev)…"
npm run dev > "$SCRIPT_DIR/vite.log" 2>&1 &
VITE_PID=$!

# ─── 2) Launch Rust server ─────────────────────────────────────────────────
cd "$SCRIPT_DIR"
LOGFILE="$SCRIPT_DIR/cargo.log"
echo "🦀 Starting Rust server… (logging to $LOGFILE)"
cargo run -- --no-can > "$LOGFILE" 2>&1 &
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
python mock_sender.py >/dev/null 2>&1 &
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
