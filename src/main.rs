extern crate modular_bitfield;
extern crate paperclip;

#[macro_use]
mod can_types;
mod boat_data_types;
mod boat_state;
mod can;
mod cli;
mod logger;
mod server;
mod websocket;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;

    let tx = websocket::MANAGER.lock().unwrap().get_sender();
    tokio::spawn(can::run(tx));
    tokio::spawn(websocket::run());

    server::run().await
}
