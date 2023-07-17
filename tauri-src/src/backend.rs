use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANFrame, CANSocket};
use tracing::*;

use tokio::sync::mpsc;

use crate::can_types;
use crate::cli;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct BoatData {
    motor_d: Option<f32>,
    bat_ii: Option<f32>,
    bat_io: Option<f32>,
}

pub async fn run_backend(tx: mpsc::Sender<BoatData>) -> Result<()> {
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

fn process_frame(frame: CANFrame) -> Result<BoatData> {
    trace!("Received CAN frame: {frame:?}");

    let data = frame.data();

    match frame.id() {
        can_types::modules::mic19::messages::motor::ID => {
            read_motor(data).map_err(anyhow::Error::from)
        }
        can_types::modules::msc19_4::messages::adc::ID => {
            read_bat_ii(data).map_err(anyhow::Error::from)
        }
        can_types::modules::msc19_5::messages::adc::ID => {
            read_bat_io(data).map_err(anyhow::Error::from)
        }
        // can_types::modules::mic19::messages::pumps::ID => todo!(),
        _ => Err(anyhow!("Unknown message")),
    }
}

fn read_motor(data: &[u8]) -> Result<BoatData, ReadMessageError> {
    trace!("Message received, trying to deserialize...: {data:?}");

    let Ok(message) =
        bincode::deserialize::<can_types::modules::mic19::messages::motor::Message>(data) else {
            return Err(ReadMessageError::DeserializationError);
        };

    if message.signature != can_types::modules::mic19::SIGNATURE {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    let boat_data = BoatData {
        motor_d: Some(100f32 * (message.d as f32) / (u8::MAX as f32)),
        ..Default::default()
    };

    debug!("Message sent: {boat_data:?}");

    Ok(boat_data)
}

fn read_bat_ii(data: &[u8]) -> Result<BoatData, ReadMessageError> {
    trace!("Message received, trying to deserialize...: {data:?}");

    let Ok(message) =
        bincode::deserialize::<can_types::modules::msc19_4::messages::adc::Message>(data) else {
            return Err(ReadMessageError::DeserializationError);
        };

    if message.signature != can_types::modules::msc19_4::SIGNATURE {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    let boat_data = BoatData {
        bat_ii: Some(100f32 * (message.average as f32) / (u16::MAX as f32)),
        ..Default::default()
    };

    debug!("Message sent: {boat_data:?}");

    Ok(boat_data)
}

fn read_bat_io(data: &[u8]) -> Result<BoatData, ReadMessageError> {
    trace!("Message received, trying to deserialize...: {data:?}");

    let Ok(message) =
        bincode::deserialize::<can_types::modules::msc19_5::messages::adc::Message>(data) else {
            return Err(ReadMessageError::DeserializationError);
        };

    if message.signature != can_types::modules::msc19_5::SIGNATURE {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    let boat_data = BoatData {
        bat_io: Some(100f32 * (message.average as f32) / (u16::MAX as f32)),
        ..Default::default()
    };

    debug!("Message sent: {boat_data:?}");

    Ok(boat_data)
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
