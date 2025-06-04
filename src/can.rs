// src/can.rs

use anyhow::{anyhow, Result};
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
        // 1) Determine which CAN interface to open
        let interface = match &cli::CONFIGURATION.can_interface {
            Some(iface) => iface.to_owned(),
            None => match lookup_can_interface() {
                Ok(found) => found,
                Err(error) => {
                    warn!(
                        "Failed to find a CAN interface; will retry in 1s… Reason: {error:?}"
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            },
        };

        // 2) Open the socket
        let mut socket_rx = CANSocket::open(&interface)?;
        debug!("Listening for CAN frames on interface \"{}\"", interface);

        let mut last_emit = std::time::Instant::now();

        // 3) Read frames in a loop, with a 1s receive timeout
        while let Some(maybe_frame) = match tokio::time::timeout(
            std::time::Duration::from_secs(1),
            socket_rx.next(),
        )
        .await
        {
            Ok(stream_item) => stream_item,
            Err(_) => {
                debug!("Receive timeout (1s) on CAN socket; restarting socket…");
                None
            }
        } {
            let frame = match maybe_frame {
                Ok(f) => f,
                Err(err) => {
                    error!(
                        "Error while reading from CAN socket; dropping this connection: {err:?}"
                    );
                    // drop + break out to outer loop, so we re-open the interface
                    drop(socket_rx);
                    break;
                }
            };

            // 4) Process the incoming CAN frame
            if let Err(err) = process_frame(frame) {
                trace!("Failed to decode one CAN frame: {err:?}");
                // Continue reading frames even if one failed
                continue;
            }

            // 5) Periodically (every CONFIGURATION.period ms) broadcast entire state
            if last_emit.elapsed() < std::time::Duration::from_millis(CONFIGURATION.period) {
                continue;
            }
            last_emit = std::time::Instant::now();

            // If no listeners, skip encoding/sending
            if tx.receiver_count() == 0 {
                continue;
            }

            // 6) Build a snapshot of BoatState → BoatData
            let boat_data: BoatData = {
                let state_snapshot = BOAT_STATE.lock().unwrap().clone();
                // ───> If/when you add per-field timestamps to BoatState, this is
                // the place to “prune” any fields older than threshold by setting
                // them to None. Since BoatData’s `From<BoatState>` already wraps
                // each field in `Option<…>`, you would override stale fields here.
                state_snapshot.into()
            };

            trace!("Broadcasting BoatData over WebSocket: {boat_data:?}");

            if let Err(send_err) = tx.send(boat_data) {
                error!("Failed to send BoatData on broadcast channel: {send_err:?}");
            }
        }
    }
}

#[instrument(level = "debug")]
fn process_frame(frame: CANFrame) -> Result<()> {
    trace!("Received CANFrame: {frame:?}");

    let data = frame.data();

    match frame.id() {
        // ── MIC ──
        modules::mic19::messages::motor::ID => {
            read_message::<modules::mic19::messages::motor::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::mde::ID => {
            read_message::<modules::mic19::messages::mde::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::mcs::ID => {
            read_message::<modules::mic19::messages::mcs::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::pumps::ID => {
            read_message::<modules::mic19::messages::pumps::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // ── MAM ──
        modules::mam19::messages::motor::ID => {
            read_message::<modules::mam19::messages::motor::Message>(data, &modules::mam19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // ── STATES (generic u8 state + error) ──
        modules::mam19::messages::state::ID => {
            read_message::<modules::mam19::messages::state::Message>(data, &modules::mam19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mic19::messages::state::ID => {
            read_message::<modules::mic19::messages::state::Message>(data, &modules::mic19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mcs19::messages::state::ID => {
            read_message::<modules::mcs19::messages::state::Message>(data, &modules::mcs19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mac22::messages::state::ID => {
            read_message::<modules::mac22::messages::state::Message>(data, &modules::mac22::SIGNATURE)
                .map_err(anyhow::Error::from)
        }
        modules::mde22::messages::state::ID => {
            read_message::<modules::mde22::messages::state::Message>(data, &modules::mde22::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // ── MCS-specific battery info ──
        modules::mcs19::messages::bat::ID => {
            read_message::<modules::mcs19::messages::bat::Message>(data, &modules::mcs19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        modules::mcs19::messages::cap::ID => {
            read_message::<modules::mcs19::messages::cap::Message>(data, &modules::mcs19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // ── MT (RPM) ──
        modules::mt19::messages::rpm::ID => {
            read_message::<modules::mt19::messages::rpm::Message>(data, &modules::mt19::SIGNATURE)
                .map_err(anyhow::Error::from)
        }

        // ── MCB (two instances) ──
        modules::mcb19_1::messages::measurements::ID => {
            read_message::<modules::mcb19_1::messages::measurements::Message>(
                data,
                &modules::mcb19_1::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }
        modules::mcb19_2::messages::measurements::ID => {
            read_message::<modules::mcb19_2::messages::measurements::Message>(
                data,
                &modules::mcb19_2::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }

        // ── MSC (five instances: indices 1–5; here only adc messages) ──
        modules::msc19_1::messages::adc::ID => {
            read_message::<modules::msc19_1::messages::adc::Message>(
                data,
                &modules::msc19_1::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }
        modules::msc19_2::messages::adc::ID => {
            read_message::<modules::msc19_2::messages::adc::Message>(
                data,
                &modules::msc19_2::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }
        modules::msc19_3::messages::adc::ID => {
            read_message::<modules::msc19_3::messages::adc::Message>(
                data,
                &modules::msc19_3::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }
        modules::msc19_4::messages::adc::ID => {
            read_message::<modules::msc19_4::messages::adc::Message>(
                data,
                &modules::msc19_4::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }
        modules::msc19_5::messages::adc::ID => {
            read_message::<modules::msc19_5::messages::adc::Message>(
                data,
                &modules::msc19_5::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }

        // ── MDE (steering/battery) ──
        modules::mde22::messages::steeringbat_measurements::ID => {
            read_message::<modules::mde22::messages::steeringbat_measurements::Message>(
                data,
                &modules::mde22::SIGNATURE,
            )
            .map_err(anyhow::Error::from)
        }

        other => Err(anyhow!("Unknown CAN message ID: 0x{:X}", other)),
    }
}

/// Attempt to decode a single generic CAN‐payload into its `BoatStateVariable` and update the global lock.
#[instrument(level = "debug")]
fn read_message<T>(data: &[u8], signature: &u8) -> Result<(), ReadMessageError>
where
    T: modules::CanMessageTrait
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>
        + std::fmt::Debug
        + BoatStateVariable,
{
    trace!("Deserializing CAN payload: {data:?}");

    let message: T = bincode::deserialize(data).map_err(|_| ReadMessageError::DeserializationError)?;

    if &message.signature() != signature {
        return Err(ReadMessageError::WrongSignatureError);
    }

    // Update the shared BoatState
    <T as BoatStateVariable>::update(message);
    Ok(())
}

enum ReadMessageError {
    DeserializationError,
    WrongSignatureError,
}

impl From<ReadMessageError> for anyhow::Error {
    fn from(err: ReadMessageError) -> Self {
        match err {
            ReadMessageError::DeserializationError => anyhow!("Deserialization error in CAN frame"),
            ReadMessageError::WrongSignatureError => anyhow!("Signature mismatch in CAN frame"),
        }
    }
}

/// If no `--can-interface` was passed, look for any “can*/vcan*” in `/sys/class/net`
#[instrument(level = "debug")]
fn lookup_can_interface() -> Result<String> {
    let sys_path = "/sys/class/net";
    for entry in std::fs::read_dir(sys_path)? {
        if let Ok(e) = entry {
            if let Some(name_os) = e.path().file_name() {
                let name = name_os.to_string_lossy();
                if name.starts_with("can") || name.starts_with("vcan") {
                    debug!("Automatically selected CAN interface: {name}");
                    return Ok(name.to_string());
                }
            }
        }
    }
    Err(anyhow!("No CAN interface found in {sys_path}"))
}
