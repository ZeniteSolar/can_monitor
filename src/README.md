# Rust Backend Changes for Local Mock Data Injection

This document describes the minimal modifications made to the `can_monitor` project to allow injecting mock `BoatData` via WebSocket — useful for local development without a CAN interface.

---

## Purpose

- Enable running the frontend and backend locally without CAN hardware
- Allow mock data to be sent (e.g., via Python) and visualized in the interface
- Preserve production behavior (CAN continues to work when not in `--no-can` mode)

---

## Changes by File

### `cli.rs`

**Added:**
```rust
/// Skips CAN interface initialization (for local dev)
#[arg(long, default_value_t = false)]
pub no_can: bool;
```
- Adds a --no-can command-line flag to skip CAN interface setup.
- Integrated into the existing Configuration struct via clap.

### `main.rs`
**Added:**
```rust
if !cli::CONFIGURATION.no_can {
    tokio::spawn(can::run(tx));
} else {
    println!("🚧 CAN interface disabled by --no-can flag (running in simulation mode)");
}
```
- Conditionally bypass CAN logic when --no-can is passed.
- Allows server and WebSocket to run normally in simulation mode.

### `websocket.rs`
**Added:**
```rust
let tx = MANAGER.lock().unwrap().get_sender();

let receive_task = tokio::spawn(async move {
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(tungstenite::Message::Text(text)) => {
                if let Ok(data) = serde_json::from_str::<BoatData>(&text) {
                    if let Err(err) = tx.send(data) {
                        error!("Failed to broadcast received BoatData: {err:?}");
                    }
                }
            }
            Ok(_) => {} // Ignore non-text
            Err(e) => {
                error!("WebSocket receive error: {:?}", e);
                break;
            }
        }
    }
});
```
- Deserializes inbound JSON Text messages into BoatData
- Broadcasts that data to the frontend via the same mechanism as can::run()

---
## Usage

Start backend in mock mode:

cargo run -- --no-can

Then send mock data using a Python script or WebSocket tool:

python3 mock_sender.py
