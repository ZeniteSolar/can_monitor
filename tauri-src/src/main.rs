// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod can_types;
mod backend;
mod cli;
mod frontend;
mod logger;

use anyhow::Result;
use tauri::Manager;
use tokio::sync::mpsc;
use tracing::*;

struct AsyncProcInputTx {
    inner: tokio::sync::Mutex<mpsc::Sender<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;

    let (tx, rx) = mpsc::channel::<can_types::modules::Messages>(100);
    let backend = tokio::spawn(backend::run_backend(tx));

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .manage(AsyncProcInputTx {
            inner: tokio::sync::Mutex::new(async_proc_input_tx),
        })
        .invoke_handler(tauri::generate_handler![front2back])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        back2front(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let frontend = tokio::spawn(frontend::run_frontend(rx));

    error!("AAAAAAAAAAA");

    tokio::signal::ctrl_c().await?;
    info!("ctrl-c received!");

    Ok(())
}

fn back2front<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    info!(?message, "back2front");
    manager
        .emit_all("back2front", format!("rs: {}", message))
        .unwrap();
}

#[tauri::command]
async fn front2back(
    message: String,
    state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    info!(?message, "front2back");
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        output_tx.send(output).await?;
    }

    Ok(())
}
