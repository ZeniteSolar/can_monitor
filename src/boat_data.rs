use serde::{Deserialize, Serialize};

use crate::boat_state::{BoatState, BoatStateVariable, Ema, BOAT_STATE};
use crate::can_types::modules;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoatData {
    boat_on: bool,
    motor_on: bool,
    motor_rev: bool,
    dms_on: bool,
    pump: [bool; 3],
    motor_d: [f32; 2],
    motor_rpm: f32,
    mam_machine_state: u8,
    mic_machine_state: u8,
    mcs_machine_state: u8,
    mac_machine_state: u8,
    mde_machine_state: u8,
    mam_error_code: u8,
    mic_error_code: u8,
    mcs_error_code: u8,
    mac_error_code: u8,
    mde_error_code: u8,
    bat_v: f32,
    bat_cell_v: [f32; 3],
    bat_ii: f32,
    bat_io: f32,
    bat_i: f32,
    bat_p: f32,
    dir_bat_v: f32,
    dir_bat_i: f32,
    dir_bat_p: f32,
    dir_pos: [f32; 2],
    mcb_d: [f32; 2],
    mcb_vi: [f32; 2],
    mcb_io: [f32; 2],
    mcb_vo: [f32; 2],
    mcb_po: [f32; 2],
    mcc_d: [f32; 9],
    mcc_ii: [f32; 9],
    mcc_vi: [f32; 9],
    mcc_vo: [f32; 9],
    mcc_pi: [f32; 9],
}

impl From<BoatState> for BoatData {
    fn from(value: BoatState) -> Self {
        let motor_d = value.motor_d.map(Ema::value);

        let bat_v = value.bat_v.value();
        let bat_cell_v = value.bat_cell_v.map(Ema::value);

        let bat_ii = value.bat_ii.value();
        let bat_io = value.bat_io.value();
        let bat_i = bat_ii - bat_io;
        let bat_p = bat_i * bat_v;

        let dir_bat_v = value.dir_bat_v.value();
        let dir_bat_i = value.dir_bat_i.value();
        let dir_bat_p = dir_bat_v * dir_bat_i;
        let dir_pos = value.dir_pos.map(Ema::value);

        let mcb_d = value.mcb_d.map(Ema::value);
        let mcb_vi = value.mcb_vi.map(Ema::value);
        let mcb_io = value.mcb_io.map(Ema::value);
        let mcb_vo = value.mcb_vo.map(Ema::value);
        let mcb_po: [f32; 2] = mcb_io
            .iter()
            .zip(mcb_vo)
            .map(|(io, vo)| io * vo)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();

        let mcc_d = value.mcc_d.map(Ema::value);
        let mcc_ii = value.mcc_ii.map(Ema::value);
        let mcc_vi = value.mcc_vi.map(Ema::value);
        let mcc_vo = value.mcc_vo.map(Ema::value);
        let mcc_pi: [f32; 9] = mcc_ii
            .iter()
            .zip(mcc_vi)
            .map(|(ii, vi)| ii * vi)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();

        Self {
            boat_on: value.boat_on,
            motor_on: value.motor_on,
            motor_rev: value.motor_rev,
            dms_on: value.dms_on,
            pump: value.pump,
            motor_d,
            motor_rpm: value.motor_rpm.value(),
            mam_machine_state: value.mam_machine_state,
            mic_machine_state: value.mic_machine_state,
            mcs_machine_state: value.mcs_machine_state,
            mac_machine_state: value.mac_machine_state,
            mde_machine_state: value.mde_machine_state,
            mam_error_code: value.mam_error_code,
            mic_error_code: value.mic_error_code,
            mcs_error_code: value.mcs_error_code,
            mac_error_code: value.mac_error_code,
            mde_error_code: value.mde_error_code,
            bat_v,
            bat_cell_v,
            bat_ii,
            bat_io,
            bat_i,
            bat_p,
            dir_bat_v,
            dir_bat_i,
            dir_bat_p,
            dir_pos,
            mcb_d,
            mcb_vi,
            mcb_io,
            mcb_vo,
            mcb_po,
            mcc_d,
            mcc_ii,
            mcc_vi,
            mcc_vo,
            mcc_pi,
        }
    }
}

impl BoatStateVariable for modules::mic19::messages::motor::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.motor_d[0].update(100f32 * (message.d as f32) / (u8::MAX as f32));

        boat_state.motor_on = message.motor.motor_on();
        boat_state.dms_on = message.motor.dms_on();
        boat_state.motor_rev = message.motor.reverse();
    }
}

impl BoatStateVariable for modules::mam19::messages::motor::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.motor_d[1].update((message.duty_cycle as f32) / 100f32);
    }
}

// Ref: https://github.com/ZeniteSolar/MAM17/blob/DSB22/firmware/src/machine.h#L34
impl BoatStateVariable for modules::mam19::messages::state::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mam_machine_state = message.state;
        boat_state.mam_error_code = message.error;
    }
}
impl BoatStateVariable for modules::mic19::messages::state::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mic_machine_state = message.state;
        boat_state.mic_error_code = message.error;
    }
}
impl BoatStateVariable for modules::mcs19::messages::state::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcs_machine_state = message.state;
        boat_state.mcs_error_code = message.error;
    }
}
impl BoatStateVariable for modules::mac22::messages::state::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mac_machine_state = message.state;
        boat_state.mac_error_code = message.error;
    }
}
impl BoatStateVariable for modules::mde22::messages::state::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mde_machine_state = message.state;
        boat_state.mde_error_code = message.error;
    }
}

impl BoatStateVariable for modules::mcs19::messages::bat::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_v.update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mic19::messages::mde::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.dir_pos[0]
            .update((26.392_962_f32 * ((message.position as f32) / 100f32)) - 135f32);
    }
}

impl BoatStateVariable for modules::mic19::messages::pumps::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.pump[0] = message.pumps.pump1();
        boat_state.pump[1] = message.pumps.pump2();
        boat_state.pump[2] = message.pumps.pump3();
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

        boat_state.motor_rpm.update(message.average as f32);
    }
}

impl BoatStateVariable for modules::msc19_1::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_cell_v[0].update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::msc19_2::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_cell_v[1].update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::msc19_3::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_cell_v[2].update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::msc19_4::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_ii.update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::msc19_5::messages::adc::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.bat_io.update((message.average as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mde22::messages::steeringbat_measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state
            .dir_bat_v
            .update((message.batvoltage as f32) / 100f32);

        boat_state
            .dir_bat_i
            .update((message.batcurrent as f32) / 100f32);

        boat_state.dir_pos[1]
            .update((26.392_962_f32 * ((message.tail_position as f32) / 100f32)) - 135f32);
    }
}

impl BoatStateVariable for modules::mcb19_1::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcb_d[0].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcb_io[0].update((message.output_current as f32) / 100f32);
        boat_state.mcb_vo[0].update((message.output_voltage as f32) / 100f32);
        boat_state.mcb_vi[0].update((message.input_voltage as f32) / 100f32);
    }
}
impl BoatStateVariable for modules::mcb19_2::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcb_d[1].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcb_io[1].update((message.output_current as f32) / 100f32);
        boat_state.mcb_vo[1].update((message.output_voltage as f32) / 100f32);
        boat_state.mcb_vi[1].update((message.input_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_1::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[0].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[0].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[0].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[0].update((message.output_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_2::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[1].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[1].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[1].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[1].update((message.output_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_3::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[2].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[2].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[2].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[2].update((message.output_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_4::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[3].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[3].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[3].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[3].update((message.output_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_5::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[4].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[4].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[4].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[4].update((message.output_voltage as f32) / 100f32);
    }
}

impl BoatStateVariable for modules::mcc19_6::messages::measurements::Message {
    fn update(message: Self) {
        let mut boat_state = BOAT_STATE.lock().unwrap();

        boat_state.mcc_d[5].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
        boat_state.mcc_ii[5].update((message.input_current as f32) / 100f32);
        boat_state.mcc_vi[5].update((message.input_voltage as f32) / 100f32);
        boat_state.mcc_vo[5].update((message.output_voltage as f32) / 100f32);
    }
}

// impl BoatStateVariable for modules::mcc19_7::messages::measurements::Message {
//     fn update(message: Self) {
//         let mut boat_state = BOAT_STATE.lock().unwrap();

//         boat_state.mcc_d[6].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
//         boat_state.mcc_ii[6].update((message.input_current as f32) / 100f32);
//         boat_state.mcc_vi[6].update((message.input_voltage as f32) / 100f32);
//         boat_state.mcc_vo[6].update((message.output_voltage as f32) / 100f32);
//     }
// }
//
// impl BoatStateVariable for modules::mcc19_8::messages::measurements::Message {
//     fn update(message: Self) {
//         let mut boat_state = BOAT_STATE.lock().unwrap();

//         boat_state.mcc_d[7].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
//         boat_state.mcc_ii[7].update((message.input_current as f32) / 100f32);
//         boat_state.mcc_vi[7].update((message.input_voltage as f32) / 100f32);
//         boat_state.mcc_vo[7].update((message.output_voltage as f32) / 100f32);
//     }
// }
//
// impl BoatStateVariable for modules::mcc19_9::messages::measurements::Message {
//     fn update(message: Self) {
//         let mut boat_state = BOAT_STATE.lock().unwrap();

//         boat_state.mcc_d[8].update(100f32 * (message.dt as f32) / (u8::MAX as f32));
//         boat_state.mcc_ii[8].update((message.input_current as f32) / 100f32);
//         boat_state.mcc_vi[8].update((message.input_voltage as f32) / 100f32);
//         boat_state.mcc_vo[8].update((message.output_voltage as f32) / 100f32);
//     }
// }
