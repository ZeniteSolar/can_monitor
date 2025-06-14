// boat_data_types.rs

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// XXX Unused, apparently...
use crate::boat_state::{BoatState};//, BOAT_STATE, Ema};
// use crate::can_types::modules;

// use std::time::Instant;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoatData {
    // ── Boolean flags ─────────────────────────────────────────────────────────
    pub boat_on:    Option<bool>,
    pub motor_on:   Option<bool>,
    pub motor_rev:  Option<bool>,
    pub dms_on:     Option<bool>,
    pub pump:       Option<[bool; 3]>,

    // ── EMA‐filtered numeric measurements ───────────────────────────────────
    pub motor_d:    Option<[f32; 2]>,
    pub motor_rpm:  Option<f32>,

    pub bat_v:      Option<f32>,
    pub bat_cell_v: Option<[f32; 3]>,
    pub bat_ii:     Option<f32>,
    pub bat_io:     Option<f32>,
    pub bat_i:      Option<f32>, // derived: bat_ii - bat_io
    pub bat_p:      Option<f32>, // derived: bat_i * bat_v

    pub dir_bat_v:  Option<f32>,
    pub dir_bat_i:  Option<f32>,
    pub dir_bat_p:  Option<f32>, // derived: dir_bat_v * dir_bat_i
    pub dir_pos:    Option<[f32; 2]>,

    pub mcb_d:      Option<[f32; 2]>,
    pub mcb_vi:     Option<[f32; 2]>,
    pub mcb_io:     Option<[f32; 2]>,
    pub mcb_vo:     Option<[f32; 2]>,
    pub mcb_po:     Option<[f32; 2]>, // derived: mcb_io * mcb_vo elementwise

    // ── Machine‐state + error codes ─────────────────────────────────────────
    // Single‐unit modules:
    pub mic_machine_state: Option<u8>,
    pub mic_error_code:    Option<u8>,

    pub mcs_machine_state: Option<u8>,
    pub mcs_error_code:    Option<u8>,

    pub mam_machine_state: Option<u8>,
    pub mam_error_code:    Option<u8>,

    pub mac_machine_state: Option<u8>,
    pub mac_error_code:    Option<u8>,

    pub mde_machine_state: Option<u8>,
    pub mde_error_code:    Option<u8>,

    // Multi‐unit modules:
    pub msc_machine_state:   Option<[u8; 5]>,
    pub msc_error_code:      Option<[u8; 5]>,

    pub mcb_machine_state:   Option<[u8; 2]>,
    pub mcb_error_code:      Option<[u8; 2]>,

    // ───────────────────────────────────────────────────────────
    // ── NEW: MCS-19 “bat” fields ──
    pub mcs_bat_avg: Option<f32>,
    pub mcs_bat_min: Option<f32>,
    pub mcs_bat_max: Option<f32>,

    // ── NEW: MCS-19 “cap” fields ──
    pub mcs_cap_avg: Option<f32>,
    pub mcs_cap_min: Option<f32>,
    pub mcs_cap_max: Option<f32>,
}

impl From<BoatState> for BoatData {
    fn from(state: BoatState) -> Self {
        // booleans are never None
        let boat_on   = Some(state.boat_on);
        let motor_on  = Some(state.motor_on);
        let motor_rev = Some(state.motor_rev);
        let dms_on    = Some(state.dms_on);
        let pump      = Some(state.pump);

        // EMAs → actual values
        let motor_d = Some([
            state.motor_d[0].value(),
            state.motor_d[1].value(),
        ]);
        let motor_rpm = Some(state.motor_rpm.value());

        let bat_v      = Some(state.bat_v.value());
        let bat_cell_v = Some([
            state.bat_cell_v[0].value(),
            state.bat_cell_v[1].value(),
            state.bat_cell_v[2].value(),
        ]);
        let bat_ii = Some(state.bat_ii.value());
        let bat_io = Some(state.bat_io.value());
        let bat_i  = Some(bat_ii.unwrap_or(0.0) - bat_io.unwrap_or(0.0));
        let bat_p  = Some(bat_i.unwrap_or(0.0) * bat_v.unwrap_or(0.0));

        let dir_bat_v = Some(state.dir_bat_v.value());
        let dir_bat_i = Some(state.dir_bat_i.value());
        let dir_bat_p = Some(dir_bat_v.unwrap_or(0.0) * dir_bat_i.unwrap_or(0.0));
        let dir_pos   = Some([
            state.dir_pos[0].value(),
            state.dir_pos[1].value(),
        ]);

        let mcb_d  = Some([state.mcb_d[0].value(), state.mcb_d[1].value()]);
        let mcb_vi = Some([state.mcb_vi[0].value(), state.mcb_vi[1].value()]);
        let mcb_io = Some([state.mcb_io[0].value(), state.mcb_io[1].value()]);
        let mcb_vo = Some([state.mcb_vo[0].value(), state.mcb_vo[1].value()]);
        let mcb_po = Some([
            mcb_io.unwrap_or([0.0, 0.0])[0] * mcb_vo.unwrap_or([0.0, 0.0])[0],
            mcb_io.unwrap_or([0.0, 0.0])[1] * mcb_vo.unwrap_or([0.0, 0.0])[1],
        ]);

        // ───────────────────────────────────────────────────────────
        // ── NEW: extract the 6 MCS fields as f32
        let mcs_bat_avg = Some(state.mcs_bat_avg.value());
        let mcs_bat_min = Some(state.mcs_bat_min.value());
        let mcs_bat_max = Some(state.mcs_bat_max.value());

        let mcs_cap_avg = Some(state.mcs_cap_avg.value());
        let mcs_cap_min = Some(state.mcs_cap_min.value());
        let mcs_cap_max = Some(state.mcs_cap_max.value());

        // Single‐unit machine states and errors (copy Option<u8>)
        let mic_machine_state = state.mic_machine_state;
        let mic_error_code    = state.mic_error_code;

        let mcs_machine_state = state.mcs_machine_state;
        let mcs_error_code    = state.mcs_error_code;

        let mam_machine_state = state.mam_machine_state;
        let mam_error_code    = state.mam_error_code;

        let mac_machine_state = state.mac_machine_state;
        let mac_error_code    = state.mac_error_code;

        let mde_machine_state = state.mde_machine_state;
        let mde_error_code    = state.mde_error_code;

        // Multi‐unit: turn [Option<u8>; N] → Option<[u8; N]>
        let msc_machine_state = {
            let arr_opt = state.msc_machine_state;
            // If *all* slots are still None, emit None
            if arr_opt.iter().all(|&o| o.is_none()) {
                None
            } else {
                // Otherwise build an array, using 0 (or another sentinel) for any missing slot
                Some([
                    arr_opt[0].unwrap_or(0),
                    arr_opt[1].unwrap_or(0),
                    arr_opt[2].unwrap_or(0),
                    arr_opt[3].unwrap_or(0),
                    arr_opt[4].unwrap_or(0),
                ])
            }
        };
        
        let msc_error_code = {
            let arr_opt = state.msc_error_code;
            if arr_opt.iter().all(|&o| o.is_some()) {
                Some([
                    arr_opt[0].unwrap(),
                    arr_opt[1].unwrap(),
                    arr_opt[2].unwrap(),
                    arr_opt[3].unwrap(),
                    arr_opt[4].unwrap(),
                ])
            } else if arr_opt.iter().all(|&o| o.is_none()) {
                None
            } else {
                None
            }
        };

        let mcb_machine_state = {
            let arr_opt = state.mcb_machine_state;
            if arr_opt.iter().all(|&o| o.is_some()) {
                Some([arr_opt[0].unwrap(), arr_opt[1].unwrap()])
            } else if arr_opt.iter().all(|&o| o.is_none()) {
                None
            } else {
                None
            }
        };
        let mcb_error_code = {
            let arr_opt = state.mcb_error_code;
            if arr_opt.iter().all(|&o| o.is_some()) {
                Some([arr_opt[0].unwrap(), arr_opt[1].unwrap()])
            } else if arr_opt.iter().all(|&o| o.is_none()) {
                None
            } else {
                None
            }
        };

        BoatData {
            boat_on,
            motor_on,
            motor_rev,
            dms_on,
            pump,
            motor_d,
            motor_rpm,
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
            mcs_bat_avg,
            mcs_bat_min,
            mcs_bat_max,
            mcs_cap_avg,
            mcs_cap_min,
            mcs_cap_max,
            mic_machine_state,
            mic_error_code,
            mcs_machine_state,
            mcs_error_code,
            mam_machine_state,
            mam_error_code,
            mac_machine_state,
            mac_error_code,
            mde_machine_state,
            mde_error_code,
            msc_machine_state,
            msc_error_code,
            mcb_machine_state,
            mcb_error_code,
        }
    }
}

// ----------------------------------------------------------------------------
// 7) OPTIONAL: If you want to implement `BoatStateVariable for some Message types that
//    produce BoatData directly (rare), you can do so here. Usually not needed.
// ----------------------------------------------------------------------------

// e.g.
// impl BoatDataVariable for modules::mic19::messages::motor::Message {
//     fn into_boat_data(self) -> BoatData { /* … */ }
// }
