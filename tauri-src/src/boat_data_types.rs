use serde::{Deserialize, Serialize};

use crate::boat_state::{BoatState, BoatStateVariable, BOAT_STATE};
use crate::can_types::modules;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct BoatData {
    boat_on: bool,
    motor_on: bool,
    pump1: bool,
    pump2: bool,
    pump3: bool,
    // TODO: mam state
    // TODO: MPPTs
    motor_d: f32,
    motor_rpm: f32,
    bat_v: f32,
    bat1_v: f32,
    bat2_v: f32,
    bat3_v: f32,
    bat_ii: f32,
    bat_io: f32,
    dir_head_pos: f32,
    dir_bat_v: f32,
    dir_bat_i: f32,
    dir_tail_pos: f32,
}

impl From<BoatState> for BoatData {
    fn from(value: BoatState) -> Self {
        Self {
            boat_on: value.boat_on,
            motor_on: value.motor_on,
            pump1: value.pump1,
            pump2: value.pump2,
            pump3: value.pump3,
            motor_d: value.motor_d.value(),
            motor_rpm: value.motor_rpm.value(),
            bat_v: value.bat_v.value(),
            bat1_v: value.bat1_v.value(),
            bat2_v: value.bat2_v.value(),
            bat3_v: value.bat3_v.value(),
            bat_ii: value.bat_ii.value(),
            bat_io: value.bat_io.value(),
            dir_head_pos: value.dir_head_pos.value(),
            dir_bat_v: value.dir_bat_v.value(),
            dir_bat_i: value.dir_bat_i.value(),
            dir_tail_pos: value.dir_tail_pos.value(),
        }
    }
}

impl BoatStateVariable for modules::mic19::messages::motor::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .motor_d
            .update(100f32 * (message.d as f32) / (u8::MAX as f32));

        boat_state.motor_on = message.motor.motor_on();
    }
}

impl BoatStateVariable for modules::mcs19::messages::bat::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat_v
            .update((message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::mic19::messages::mde::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .dir_head_pos
            .update((26.392_962_f32 * (message.position as f32 / u8::MAX as f32)) - 135f32);
    }
}

impl BoatStateVariable for modules::mic19::messages::pumps::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.pump1 = message.pumps.pump1();
        boat_state.pump2 = message.pumps.pump2();
        boat_state.pump3 = message.pumps.pump3();
    }
}

impl BoatStateVariable for modules::mic19::messages::mcs::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.boat_on = message.boat_on.boat_on();
    }
}

impl BoatStateVariable for modules::mt19::messages::rpm::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .motor_rpm
            .update((message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::msc19_1::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat1_v
            .update(100f32 * (message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::msc19_2::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat2_v
            .update(100f32 * (message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::msc19_3::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat3_v
            .update(100f32 * (message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::msc19_4::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat_ii
            .update(100f32 * (message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::msc19_5::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .bat_io
            .update(100f32 * (message.average as f32) / (u16::MAX as f32));
    }
}

impl BoatStateVariable for modules::mde22::messages::steeringbat_measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .dir_bat_v
            .update(100f32 * (message.batvoltage as f32) / (u16::MAX as f32));

        boat_state
            .dir_bat_i
            .update(100f32 * (message.batcurrent as f32) / (u16::MAX as f32));

        boat_state
            .dir_tail_pos
            .update((26.392_962_f32 * (message.tail_position as f32 / u8::MAX as f32)) - 135f32);
    }
}
