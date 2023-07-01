use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANFrame, CANSocket};
use tracing::*;

use tokio::sync::mpsc;

use crate::can_types;
use crate::cli;

pub async fn run_backend(tx: mpsc::Sender<can_types::modules::Messages>) -> Result<()> {
    loop {
        let interface = match &cli::CONFIGURATION.can_interface {
            Some(interface) => interface.to_owned(),
            None => lookup_can_interface()?,
        };

        let mut socket_rx = CANSocket::open(&interface)?;
        debug!("Reading on {interface:?}");

        while let Some(result) = socket_rx.next().await {
            let frame = match result {
                Ok(frame) => frame,
                Err(error) => {
                    error!("Failed receiving: {error:?}");
                    continue;
                }
            };

            let message = match process_frame(frame) {
                Ok(message) => message,
                Err(error) => {
                    trace!("Failed processing message: {error:?}");
                    continue;
                }
            };

            if let Err(error) = tx.send(message).await {
                error!("Failed sending message: {error:?}");
            }
        }
    }
}

fn process_frame(frame: CANFrame) -> Result<can_types::modules::Messages> {
    trace!("Received CAN frame: {frame:?}");

    let data = frame.data();

    match frame.id() {
        can_types::modules::mic19::messages::motor::ID => {
            read_motor(data, &can_types::modules::mic19::SIGNATURE).map_err(anyhow::Error::from)
        }
        can_types::modules::mic19::messages::pumps::ID => todo!(),
        _ => Err(anyhow!("Unknown message")),
    }
}

fn read_motor(
    data: &[u8],
    sender_signature: &u8,
) -> Result<can_types::modules::Messages, ReadMessageError> {
    trace!("Motor message received, trying to deserialize...: {data:?}");

    let Ok(message) =
        bincode::deserialize::<can_types::modules::mic19::messages::motor::Message>(data) else {
            return Err(ReadMessageError::DeserializationError);
        };

    let signature = &message.signature;
    if signature != sender_signature {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    Ok(can_types::modules::Messages::Mic19(
        can_types::modules::mic19::Messages::Motor(message),
    ))
}

enum ReadMessageError {
    DeserializationError,
    WrongSignatureError,
}

impl From<ReadMessageError> for anyhow::Error {
    fn from(error: ReadMessageError) -> Self {
        match error {
            ReadMessageError::DeserializationError => anyhow!("Deserialization error occurred"),
            ReadMessageError::WrongSignatureError => anyhow!("Wrong signature error occurred"),
        }
    }
}

fn lookup_can_interface() -> Result<String> {
    let system_interfaces_path = "/sys/class/net";
    let entries = std::fs::read_dir(system_interfaces_path)?;

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();
        let Some(interface_name) = path.file_name() else {
            continue;
        };
        let interface_name = interface_name.to_string_lossy();

        if !(interface_name.starts_with("can") || interface_name.starts_with("vcan")) {
            continue;
        }

        debug!("CAN Interface found: {interface_name:?}");
        return Ok(interface_name.to_string());
    }

    Err(anyhow!("No Can interfaces found"))
}
