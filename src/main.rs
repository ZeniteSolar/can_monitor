#[macro_use]
mod can_types;
mod backend;
mod cli;
mod frontend;
mod logger;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init()?;

    let (tx, rx) = mpsc::channel::<can_types::modules::Messages>(100);

    let backend = tokio::spawn(backend::run_backend(tx));

    let frontend = tokio::spawn(frontend::run_frontend(rx));

    tokio::signal::ctrl_c().await?;
    info!("ctrl-c received!");

    Ok(())
}
