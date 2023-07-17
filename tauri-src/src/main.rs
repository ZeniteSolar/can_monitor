// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate modular_bitfield;

#[macro_use]
mod can_types;
mod backend;
mod cli;
mod logger;

use anyhow::Result;
use backend::BoatData;
use tauri::Manager;
use tokio::sync::mpsc;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;

    let (tx, rx) = mpsc::channel::<BoatData>(100);
    tokio::spawn(backend::run_backend(tx));

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                send_messages_to_frontend(rx, &app_handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn send_messages_to_frontend<R: tauri::Runtime>(
    mut rx: mpsc::Receiver<BoatData>,
    manager: &impl Manager<R>,
) {
    loop {
        if let Some(message) = rx.recv().await {
            trace!("Message sent to front: {message:?}");
            manager.emit_all("can_message", message).unwrap();
        }
    }
}
