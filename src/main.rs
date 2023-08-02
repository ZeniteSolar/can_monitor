extern crate modular_bitfield;

#[macro_use]
mod can_types;
mod boat_data;
mod boat_state;
mod can;
mod cli;
mod gui;
mod logger;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;

    tokio::spawn(can::run());

    gui::run();

    Ok(())
}
