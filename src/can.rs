use anyhow::{anyhow, Result};
use futures::prelude::sink::SinkExt;
use futures_util::stream::StreamExt;
use tokio_socketcan::{CANFrame, CANSocket};
use tracing::*;

use tokio::sync::broadcast;

use crate::boat_data_types::BoatData;
use crate::boat_state::{BoatStateVariable, BOAT_STATE};
use crate::can_types::modules;
use crate::cli::{self, CONFIGURATION};

#[instrument(level = "debug")]
pub async fn run(tx: broadcast::Sender<BoatData>) -> Result<()> {
    loop {
        let interface = match &cli::CONFIGURATION.can_interface {
            Some(interface) => interface.to_owned(),
            None => match lookup_can_interface() {
                Ok(interface) => interface,
                Err(error) => {
                    warn!("Failed getting the can interface, trying again in 1 second... Reason: {error:?}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            },
        };

        let mut socket_rx = CANSocket::open(&interface)?;
        debug!("Reading on {interface:?}");

        let mut time = std::time::Instant::now();

        while let Some(result) =
            match tokio::time::timeout(std::time::Duration::from_secs(1), socket_rx.next()).await {
                Ok(next) => next,
                Err(_) => {
                    debug!("Receiving message timeout, resetting socket connection...");
                    None
                }
            }
        {
            let frame = match result {
                Ok(frame) => frame,
                Err(error) => {
                    error!(
                        "Failed receiving from can socket, dropping connection... Reason: {error:?}"
                    );
                    let _ = socket_rx.close().await;
                    break;
                }
            };

            if let Err(error) = process_frame(frame) {
                trace!("Failed processing message: {error:?}");
                continue;
            }

            if time.elapsed() < std::time::Duration::from_millis(CONFIGURATION.period) {
                continue;
            }
            time = std::time::Instant::now();

            if tx.receiver_count() == 0 {
                continue;
            }

            let message = BOAT_STATE.lock().unwrap().clone().into();

            trace!("Sending message: {message:?}");

            if let Err(error) = tx.send(message) {
                error!("Failed sending message: {error:?}");
            }
        }
    }
}

#[instrument(level = "debug")]
fn process_frame(frame: CANFrame) -> Result<()> {
    trace!("Received CAN frame: {frame:?}");

    let data = frame.data();

    match frame.id() {
        // MIC
        modules::mic19::messages::motor::ID => read_message::<
            modules::mic19::messages::motor::Message,
        >(data, &modules::mic19::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::mic19::messages::mde::ID => {
            read_message::<modules::mic19::messages::mde::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::mcs::ID => {
            read_message::<modules::mic19::messages::mcs::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::pumps::ID => read_message::<
            modules::mic19::messages::pumps::Message,
        >(data, &modules::mic19::SIGNATURE)
        .map_err(anyhow::Error::from),

        // MAM
        modules::mam19::messages::motor::ID => read_message::<
            modules::mam19::messages::motor::Message,
        >(data, &modules::mam19::SIGNATURE)
        .map_err(anyhow::Error::from),

        // STATES
        modules::mam19::messages::state::ID => read_message::<
            modules::mam19::messages::state::Message,
        >(data, &modules::mam19::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::mic19::messages::state::ID => read_message::<
            modules::mic19::messages::state::Message,
        >(data, &modules::mic19::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::mcs19::messages::state::ID => read_message::<
            modules::mcs19::messages::state::Message,
        >(data, &modules::mcs19::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::mac22::messages::state::ID => read_message::<
            modules::mac22::messages::state::Message,
        >(data, &modules::mac22::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::mde22::messages::state::ID => read_message::<
            modules::mde22::messages::state::Message,
        >(data, &modules::mde22::SIGNATURE)
        .map_err(anyhow::Error::from),

        // MCS
        modules::mcs19::messages::bat::ID => {
            read_message::<modules::mcs19::messages::bat::Message>(data, &modules::mcs19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // MT
        modules::mt19::messages::rpm::ID => {
            read_message::<modules::mt19::messages::rpm::Message>(data, &modules::mt19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // MCB
        modules::mcb19_1::messages::measurements::ID => read_message::<
            modules::mcb19_1::messages::measurements::Message,
        >(
            data, &modules::mcb19_1::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcb19_2::messages::measurements::ID => read_message::<
            modules::mcb19_2::messages::measurements::Message,
        >(
            data, &modules::mcb19_2::SIGNATURE
        )
        .map_err(anyhow::Error::from),

        // MSC
        modules::msc19_1::messages::adc::ID => read_message::<
            modules::msc19_4::messages::adc::Message,
        >(data, &modules::msc19_1::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_2::messages::adc::ID => read_message::<
            modules::msc19_4::messages::adc::Message,
        >(data, &modules::msc19_2::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_3::messages::adc::ID => read_message::<
            modules::msc19_4::messages::adc::Message,
        >(data, &modules::msc19_3::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_4::messages::adc::ID => read_message::<
            modules::msc19_4::messages::adc::Message,
        >(data, &modules::msc19_4::SIGNATURE)
        .map_err(anyhow::Error::from),
        modules::msc19_5::messages::adc::ID => read_message::<
            modules::msc19_5::messages::adc::Message,
        >(data, &modules::msc19_5::SIGNATURE)
        .map_err(anyhow::Error::from),

        // MDE
        modules::mde22::messages::steeringbat_measurements::ID => {
            read_message::<modules::mde22::messages::steeringbat_measurements::Message>(
                data,
                &modules::mde22::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }

        // MCC
        modules::mcc23_1::messages::measurements::ID => read_message::<
            modules::mcc23_1::messages::measurements::Message,
        >(
            data, &modules::mcc23_1::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_2::messages::measurements::ID => read_message::<
            modules::mcc23_2::messages::measurements::Message,
        >(
            data, &modules::mcc23_2::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_3::messages::measurements::ID => read_message::<
            modules::mcc23_3::messages::measurements::Message,
        >(
            data, &modules::mcc23_3::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_4::messages::measurements::ID => read_message::<
            modules::mcc23_4::messages::measurements::Message,
        >(
            data, &modules::mcc23_4::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_5::messages::measurements::ID => read_message::<
            modules::mcc23_5::messages::measurements::Message,
        >(
            data, &modules::mcc23_5::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_6::messages::measurements::ID => read_message::<
            modules::mcc23_6::messages::measurements::Message,
        >(
            data, &modules::mcc23_6::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_7::messages::measurements::ID => read_message::<
            modules::mcc23_7::messages::measurements::Message,
        >(
            data, &modules::mcc23_7::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_8::messages::measurements::ID => read_message::<
            modules::mcc23_8::messages::measurements::Message,
        >(
            data, &modules::mcc23_8::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        modules::mcc23_9::messages::measurements::ID => read_message::<
            modules::mcc23_9::messages::measurements::Message,
        >(
            data, &modules::mcc23_9::SIGNATURE
        )
        .map_err(anyhow::Error::from),
        msg => Err(anyhow!("Unknown message: {msg:?}")),
    }
}

#[instrument(level = "debug")]
fn read_message<T>(data: &[u8], signature: &u8) -> Result<(), ReadMessageError>
where
    T: modules::CanMessageTrait
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + std::fmt::Debug
        + BoatStateVariable,
{
    trace!("Message received, trying to deserialize...: {data:?}");

    let Ok(message) = bincode::deserialize::<T>(data) else {
        return Err(ReadMessageError::DeserializationError);
    };

    if &message.signature() != signature {
        return Err(ReadMessageError::WrongSignatureError);
    }

    debug!("Message read: {message:?}");

    <T as BoatStateVariable>::update(message);

    Ok(())
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

#[instrument(level = "debug")]
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
