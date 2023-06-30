use anyhow::Result;
use tokio::sync::mpsc;
use tracing::*;

use crate::can_types;

pub async fn run_frontend(mut rx: mpsc::Receiver<can_types::modules::Messages>) -> Result<()> {
    loop {
        while let Some(message) = rx.recv().await {
            info!("Received: {message:#?}");
        }
    }
}
