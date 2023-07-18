use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANFrame, CANSocket};
use tracing::*;

use tokio::sync::mpsc;

use crate::boat_data_types::BoatData;
use crate::can_types::modules;
use crate::cli;

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
        modules::mic19::messages::motor::ID => read_message::<
            modules::mic19::messages::motor::Message,
        >(data, &modules::mic19::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_4::messages::adc::ID => read_message::<
            modules::msc19_4::messages::adc::Message,
        >(data, &modules::msc19_4::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_5::messages::adc::ID => read_message::<
            modules::msc19_5::messages::adc::Message,
        >(data, &modules::msc19_5::SIGNATURE)
        .map_err(anyhow::Error::from),
        // modules::mic19::messages::pumps::ID => todo!(),
        _ => Err(anyhow!("Unknown message")),
    }
}

fn read_message<T>(data: &[u8], signature: &u8) -> Result<BoatData, ReadMessageError>
where
    T: modules::CanMessageTrait
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + std::fmt::Debug
        + Into<BoatData>,
{
    trace!("Message received, trying to deserialize...: {data:?}");

    let Ok(message) =
        bincode::deserialize::<T>(data) else {
            return Err(ReadMessageError::DeserializationError);
        };

    if &message.signature() != signature {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    let data = message.into();

    Ok(data)
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
