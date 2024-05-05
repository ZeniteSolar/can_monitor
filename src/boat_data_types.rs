use serde::{Deserialize, Serialize};

use crate::boat_state::{BoatState, BoatStateVariable, Sma, BOAT_STATE};
use crate::can_types::modules;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Wrappedf32Vec {
    values: Vec<f32>,
    values_pct: Vec<f32>,
    sum: f32,
    sum_pct: f32,
    avg: f32,
    avg_pct: f32,
    units: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Wrappedf32 {
    value: f32,
    value_pct: f32,
    units: String,
}

impl Wrappedf32Vec {
    fn new(values: &[f32], units: &str, min: f32, max: f32) -> Self {
        let len = values.len();
        let mut sum = 0.0;
        let mut values_pct = Vec::with_capacity(len);

        for &v in values {
            sum += v;
            let pct = 100.0 * (v - min) / (max - min);
            values_pct.push(pct);
        }

        let values = values.to_vec();
        let units = units.to_string();

        let sum_pct = 100.0 * (sum - min) / (max - min);
        let avg = sum / (len as f32);
        let avg_pct = 100.0 * (avg - min) / (max - min);

        Self {
            values,
            values_pct,
            sum,
            sum_pct,
            avg,
            avg_pct,
            units,
        }
    }
}

impl Wrappedf32 {
    fn new(value: &f32, units: &str, min: f32, max: f32) -> Self {
        let delta = max - min;
        let value = value.to_owned();
        let value_pct = (value - min) / delta;
        let units = units.to_string();

        Self {
            value,
            value_pct,
            units,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoatData {
    boat_on: bool,
    motor_on: bool,
    motor_rev: bool,
    dms_on: bool,
    pump: [bool; 3],
    motor_d: Wrappedf32Vec,
    motor_rpm: Wrappedf32,
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
    bat_v: Wrappedf32,
    bat_cell_v: Wrappedf32Vec,
    bat_ii: Wrappedf32,
    bat_io: Wrappedf32,
    bat_i: Wrappedf32,
    bat_p: Wrappedf32,
    dir_bat_v: Wrappedf32,
    dir_bat_i: Wrappedf32,
    dir_bat_p: Wrappedf32,
    dir_pos: Wrappedf32Vec,
    mcb_d: Wrappedf32Vec,
    mcb_vi: Wrappedf32Vec,
    mcb_io: Wrappedf32Vec,
    mcb_vo: Wrappedf32Vec,
    mcb_po: Wrappedf32Vec,
    mcc_d: Wrappedf32Vec,
    mcc_ii: Wrappedf32Vec,
    mcc_vi: Wrappedf32Vec,
    mcc_vo: Wrappedf32Vec,
    mcc_pi: Wrappedf32Vec,
}

impl From<BoatState> for BoatData {
    fn from(state: BoatState) -> Self {
        let motor_d = Wrappedf32Vec::new(&state.motor_d.map(Sma::get), "%", 0.0, 100.0);

        let bat_v = Wrappedf32::new(&state.bat_v.get(), "V", 30.0, 60.0);
        let bat_cell_v = Wrappedf32Vec::new(&state.bat_cell_v.map(Sma::get), "V", 7.0, 16.5);

        let bat_ii = Wrappedf32::new(&state.bat_ii.get(), "A", 0.0, 150.0);
        let bat_io = Wrappedf32::new(&state.bat_io.get(), "A", 0.0, 150.0);
        let bat_i = Wrappedf32::new(&(bat_ii.value - bat_io.value), "A", -150.0, 150.0);
        let bat_p = Wrappedf32::new(
            &(bat_i.value * bat_v.value),
            "W",
            -150.0 * 30.0,
            150.0 * 60.0,
        );

        let dir_bat_v = Wrappedf32::new(&state.dir_bat_v.get(), "V", 7.0, 16.5);
        let dir_bat_i = Wrappedf32::new(&state.dir_bat_i.get(), "A", 0.0, 20.0);
        let dir_bat_p =
            Wrappedf32::new(&(dir_bat_i.value * dir_bat_v.value), "W", 0.0, 20.0 * 16.5);
        let dir_pos = Wrappedf32Vec::new(&state.dir_pos.map(Sma::get), "°", -135.0, 135.0);

        let mcb_d = Wrappedf32Vec::new(&state.mcb_d.map(Sma::get), "%", 0.0, 100.0);
        let mcb_vi = Wrappedf32Vec::new(&state.mcb_vi.map(Sma::get), "V", 0.0, 60.0);
        let mcb_io = state.mcb_io.map(Sma::get);
        let mcb_vo = state.mcb_vo.map(Sma::get);
        let mcb_po = mcb_io
            .iter()
            .zip(&mcb_vo)
            .map(|(io, vo)| io * vo)
            .collect::<Vec<f32>>();
        let mcb_io = Wrappedf32Vec::new(&mcb_io, "A", 0.0, 15.0);
        let mcb_vo = Wrappedf32Vec::new(&mcb_vo, "V", 0.0, 60.0);
        let mcb_po = Wrappedf32Vec::new(&mcb_po, "W", 0.0, 300.0);

        let mcc_d = Wrappedf32Vec::new(&state.mcc_d.map(Sma::get), "%", 0.0, 100.0);
        let mcc_ii = state.mcc_ii.map(Sma::get);
        let mcc_vi = state.mcc_vi.map(Sma::get);
        let mcc_pi = mcc_ii
            .iter()
            .zip(&mcc_vi)
            .map(|(ii, vi)| ii * vi)
            .collect::<Vec<f32>>();
        let mcc_ii = Wrappedf32Vec::new(&mcc_ii, "A", 0.0, 15.0);
        let mcc_vi = Wrappedf32Vec::new(&mcc_vi, "V", 0.0, 60.0);
        let mcc_pi = Wrappedf32Vec::new(&mcc_pi, "W", 0.0, 300.0);
        let mcc_vo = Wrappedf32Vec::new(&state.mcc_vo.map(Sma::get), "V", 30.0, 60.0);

        Self {
            boat_on: state.boat_on,
            motor_on: state.motor_on,
            motor_rev: state.motor_rev,
            dms_on: state.dms_on,
            pump: state.pump,
            motor_d,
            motor_rpm: Wrappedf32::new(&state.motor_rpm.get(), "rad/s", 0.0, 4800.0),
            mam_machine_state: state.mam_machine_state,
            mic_machine_state: state.mic_machine_state,
            mcs_machine_state: state.mcs_machine_state,
            mac_machine_state: state.mac_machine_state,
            mde_machine_state: state.mde_machine_state,
            mam_error_code: state.mam_error_code,
            mic_error_code: state.mic_error_code,
            mcs_error_code: state.mcs_error_code,
            mac_error_code: state.mac_error_code,
            mde_error_code: state.mde_error_code,
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

        boat_state.dir_pos[0].update((270f32 * (message.position as f32) / 1024f32) - 135f32);
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

        boat_state.dir_pos[1].update(((message.tail_position as f32) / 100f32) - 135f32);
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
