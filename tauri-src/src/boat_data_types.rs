use serde::{Deserialize, Serialize};

use crate::boat_state::BOAT_STATE;
use crate::can_types::modules;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct BoatData {
    boat_on: Option<bool>,
    motor_on: Option<bool>,
    // TODO: mam state
    // TODO: MPPTs
    motor_d: Option<f32>,
    motor_rpm: Option<f32>,
    bat_v: Option<f32>,
    bat1_v: Option<f32>,
    bat2_v: Option<f32>,
    bat3_v: Option<f32>,
    bat_ii: Option<f32>,
    bat_io: Option<f32>,
    dir_head_pos: Option<f32>,
    dir_bat_v: Option<f32>,
    dir_bat_i: Option<f32>,
    dir_tail_pos: Option<f32>,
    pump1: Option<bool>,
    pump2: Option<bool>,
    pump3: Option<bool>,
}

impl From<modules::mic19::messages::motor::Message> for BoatData {
    fn from(message: modules::mic19::messages::motor::Message) -> Self {
        Self {
            motor_d: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .motor_d
                    .update(100f32 * (message.d as f32) / (u8::MAX as f32)),
            ),
            motor_on: Some(message.motor.motor_on()),
            ..Default::default()
        }
    }
}

impl From<modules::mcs19::messages::bat::Message> for BoatData {
    fn from(message: modules::mcs19::messages::bat::Message) -> Self {
        Self {
            bat_v: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat_v
                    .update((message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::mic19::messages::mde::Message> for BoatData {
    fn from(message: modules::mic19::messages::mde::Message) -> Self {
        Self {
            dir_head_pos: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .dir_head_pos
                    .update((26.392_962_f32 * (message.position as f32 / u8::MAX as f32)) - 135f32),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::mic19::messages::pumps::Message> for BoatData {
    fn from(message: modules::mic19::messages::pumps::Message) -> Self {
        Self {
            pump1: Some(message.pumps.pump1()),
            pump2: Some(message.pumps.pump2()),
            pump3: Some(message.pumps.pump3()),
            ..Default::default()
        }
    }
}

impl From<modules::mic19::messages::mcs::Message> for BoatData {
    fn from(message: modules::mic19::messages::mcs::Message) -> Self {
        Self {
            boat_on: Some(message.boat_on.boat_on()),
            ..Default::default()
        }
    }
}

impl From<modules::mt19::messages::rpm::Message> for BoatData {
    fn from(message: modules::mt19::messages::rpm::Message) -> Self {
        Self {
            motor_rpm: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .motor_rpm
                    .update((message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_1::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_1::messages::adc::Message) -> Self {
        Self {
            bat1_v: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat1_v
                    .update(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_2::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_2::messages::adc::Message) -> Self {
        Self {
            bat2_v: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat2_v
                    .update(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_3::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_3::messages::adc::Message) -> Self {
        Self {
            bat3_v: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat3_v
                    .update(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_4::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_4::messages::adc::Message) -> Self {
        Self {
            bat_ii: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat_ii
                    .update(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_5::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_5::messages::adc::Message) -> Self {
        Self {
            bat_io: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .bat_io
                    .update(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ),
            ..Default::default()
        }
    }
}

impl From<modules::mde22::messages::steeringbat_measurements::Message> for BoatData {
    fn from(message: modules::mde22::messages::steeringbat_measurements::Message) -> Self {
        Self {
            dir_bat_v: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .dir_bat_v
                    .update(100f32 * (message.batvoltage as f32) / (u16::MAX as f32)),
            ),
            dir_bat_i: Some(
                BOAT_STATE
                    .lock()
                    .unwrap()
                    .dir_bat_i
                    .update(100f32 * (message.batcurrent as f32) / (u16::MAX as f32)),
            ),
            dir_tail_pos: Some(BOAT_STATE.lock().unwrap().dir_tail_pos.update(
                (26.392_962_f32 * (message.tail_position as f32 / u8::MAX as f32)) - 135f32,
            )),
            ..Default::default()
        }
    }
}
