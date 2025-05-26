use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tracing::*;

use crate::boat_data_types::BoatData;

#[derive(Debug)]
pub struct Manager {
    channel_sender: tokio::sync::broadcast::Sender<BoatData>,
}

impl Manager {
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self {
            channel_sender: tokio::sync::broadcast::channel(1).0,
        }
    }

    #[instrument(level = "debug")]
    pub fn get_receiver(&self) -> tokio::sync::broadcast::Receiver<BoatData> {
        self.channel_sender.subscribe()
    }

    #[instrument(level = "debug")]
    pub fn get_sender(&self) -> tokio::sync::broadcast::Sender<BoatData> {
        self.channel_sender.clone()
    }
}

lazy_static! {
    pub static ref MANAGER: Arc<Mutex<Manager>> = Arc::new(Mutex::new(Manager::new()));
}

pub const DEFAULT_WS_ENDPOINT: &str = "ws://0.0.0.0:3001";

#[derive(Debug)]
pub struct Websocket {
    _server_thread_handle: std::thread::JoinHandle<()>,
}

#[instrument(level = "debug")]
pub async fn run() -> Result<()> {
    let endpoint = match url::Url::parse(DEFAULT_WS_ENDPOINT).context("Failed parsing endpoint") {
        Ok(endpoint) => endpoint,
        Err(error) => {
            return Err(anyhow!(
                "Failed parsing TurnServer url {DEFAULT_WS_ENDPOINT:?}: {error:?}"
            ));
        }
    };

    debug!("Starting Signalling server on {endpoint:?}...");
    Websocket::runner(endpoint.clone()).await
}

impl Websocket {
    #[instrument(level = "debug")]
    async fn runner(endpoint: url::Url) -> Result<()> {
        let host = endpoint
            .host()
            .context(format!("Failed to get the host from {endpoint:#?}"))?;
        let port = endpoint
            .port()
            .context(format!("Failed to get the port from {endpoint:#?}"))?;

        let addr = format!("{host}:{port}").parse::<SocketAddr>()?;

        let listener = TcpListener::bind(&addr).await?;
        debug!("Signalling server: listening on: {addr:?}");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .context("connected streams should have a peer address")?;
            debug!("Peer address: {peer:?}");

            tokio::spawn(Self::accept_connection(peer, stream));
        }

        Ok(())
    }

    #[instrument(level = "debug")]
    async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
        type Error = tungstenite::Error;

        if let Err(e) = Self::handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            }
        }
    }

    #[instrument(level = "debug")]
    async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> tungstenite::Result<()> {
        let (mut ws_sender, mut ws_receiver) = match tokio_tungstenite::accept_async(stream).await {
            Ok(result) => result.split(),
            Err(error) => {
                error!("Failed to accept websocket connection. Reason: {error:?}");
                return Err(error);
            }
        };

        info!("New WebSocket connection: {peer:?}");

        let mut data_receiver = MANAGER.lock().unwrap().get_receiver();

        // --- START NEW: receive task for mock clients like Python ---
        let tx = MANAGER.lock().unwrap().get_sender();

        let _receive_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(tungstenite::Message::Text(text)) => {
                        match serde_json::from_str::<BoatData>(&text) {
                            Ok(data) => {
                                if let Err(err) = tx.send(data) {
                                    error!("Failed to broadcast received BoatData: {err:?}");
                                }
                            }
                            Err(err) => {
                                warn!("Invalid BoatData JSON received: {err:?}");
                            }
                        }
                    }
                    Ok(_) => {} // Ignore non-text messages
                    Err(e) => {
                        error!("WebSocket error from client: {e:?}");
                        break;
                    }
                }
            }
        });
        // --- END NEW ---

        // Existing sender loop: broadcast BoatData to frontend
        while let Ok(message) = data_receiver.recv().await {
            trace!("Sending..: {message:#?}");

            let message: tungstenite::Message = match message.try_into() {
                Ok(message) => message,
                Err(error) => {
                    error!("Failed transforming Protocol into Tungstenite' message: {error:?}");
                    break;
                }
            };

            if let Err(error) = ws_sender.send(message).await {
                error!(
                    "Failed repassing message from the MPSC to the WebSocket. Reason: {error:?}"
                );
                break;
            }
        }

        info!("MPSC channel closed.");

        if let Err(reason) = ws_sender.close().await {
            error!("Failed closing WebSocket channel. Reason: {reason}");
        }

        info!("Websocket closed.");
        Ok(())
    }
}

impl TryFrom<tungstenite::Message> for BoatData {
    type Error = anyhow::Error;

    #[instrument(level = "trace")]
    fn try_from(value: tungstenite::Message) -> Result<Self, Self::Error> {
        let msg = value.to_text()?;
        let protocol = serde_json::from_str::<BoatData>(msg)?;
        Ok(protocol)
    }
}

impl TryInto<tungstenite::Message> for BoatData {
    type Error = anyhow::Error;

    #[instrument(level = "trace", skip(self))]
    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        let json_str = serde_json::to_string(&self)?;
        Ok(tungstenite::Message::Text(json_str))
    }
}