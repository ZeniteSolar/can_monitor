// boat_state.rs

use std::sync::{Arc, Mutex};
use std::time::Instant;

use lazy_static::lazy_static;

// ----------------------------------------------------------------------------
// 1) SINGLETON BOAT_STATE
// ----------------------------------------------------------------------------

lazy_static! {
    /// Global, thread-safe container of all live boat telemetry.
    pub static ref BOAT_STATE: Arc<Mutex<BoatState>> = Default::default();
}

// ----------------------------------------------------------------------------
// 2) EMA HELPER (unchanged except for small formatting)
// ----------------------------------------------------------------------------

#[derive(Default, Debug, Clone)]
pub struct Ema<const MAX_SAMPLES: usize> {
    samples: usize,
    sum: f32,
}

impl<const MAX_SAMPLES: usize> Ema<MAX_SAMPLES> {
    /// Incorporate one new sample into the running EMA.
    pub fn update(&mut self, value: f32) {
        if self.samples < MAX_SAMPLES {
            self.samples += 1;
        }
        self.sum += (value - self.sum) / (self.samples as f32);
    }

    /// Return the current EMA value (0.0 if no samples yet).
    pub fn value(&self) -> f32 {
        self.sum
    }
}

// ----------------------------------------------------------------------------
// 3) BOAT_STATE: ALL FIELDS AS Option<…> + Instant Timestamps
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BoatState {
    // ── Boolean flags ─────────────────────────────────────────────────────────
    pub boat_on: bool,
    pub motor_on: bool,
    pub motor_rev: bool,
    pub dms_on: bool,
    pub pump: [bool; 3],

    // ── EMA‐filtered numeric measurements ───────────────────────────────────
    pub motor_d: [Ema<10>; 2], // two motors: index 0 = MIC, index 1 = MAM
    pub motor_rpm: Ema<10>,

    pub bat_v: Ema<10>,
    pub bat_cell_v: [Ema<10>; 3],
    pub bat_ii: Ema<10>,
    pub bat_io: Ema<10>,

    pub dir_pos: [Ema<10>; 2], // steering pos: index 0 (MIC), index 1 (MDE)
    pub dir_bat_v: Ema<10>,
    pub dir_bat_i: Ema<10>,

    pub mcb_vi: [Ema<10>; 2],
    pub mcb_io: [Ema<10>; 2],
    pub mcb_vo: [Ema<10>; 2],
    pub mcb_d: [Ema<10>; 2],

    // ───────────────────────────────────────────────────────────
    // MCS-19 BATTERY (“bat”) fields: we keep a running EMA of average, min, max
    pub mcs_bat_avg: Ema<10>,
    pub mcs_bat_min: Ema<10>,
    pub mcs_bat_max: Ema<10>,

    // MCS-19 CAPACITOR (“cap”) fields: EMA for average, min, max
    pub mcs_cap_avg: Ema<10>,
    pub mcs_cap_min: Ema<10>,
    pub mcs_cap_max: Ema<10>,

    // ── “MACHINE STATE” fields (Option<u8>) + error codes (Option<u8>) ──────
    // Single‐unit modules:
    pub mic_machine_state: Option<u8>,
    pub mic_error_code: Option<u8>,
    pub mic_last_seen: Option<Instant>,

    pub mcs_machine_state: Option<u8>,
    pub mcs_error_code: Option<u8>,
    pub mcs_last_seen: Option<Instant>,

    pub mam_machine_state: Option<u8>,
    pub mam_error_code: Option<u8>,
    pub mam_last_seen: Option<Instant>,

    pub mac_machine_state: Option<u8>,
    pub mac_error_code: Option<u8>,
    pub mac_last_seen: Option<Instant>,

    pub mde_machine_state: Option<u8>,
    pub mde_error_code: Option<u8>,
    pub mde_last_seen: Option<Instant>,

    // Multi‐unit modules: MSC (3 slots), MCB (2 slots)
    pub msc_machine_state: [Option<u8>; 3],
    pub msc_error_code: [Option<u8>; 3],
    pub msc_last_seen: [Option<Instant>; 3],

    pub mcb_machine_state: [Option<u8>; 2],
    pub mcb_error_code: [Option<u8>; 2],
    pub mcb_last_seen: [Option<Instant>; 2],
}

// ----------------------------------------------------------------------------
// 4) DEFAULT IMPLEMENTATION
// ----------------------------------------------------------------------------

impl Default for BoatState {
    fn default() -> Self {
        BoatState {
            // Boolean flags default to false
            boat_on: false,
            motor_on: false,
            motor_rev: false,
            dms_on: false,
            pump: [false; 3],

            // EMAs default‐construct (0 samples, sum = 0.0)
            motor_d: [Ema::default(), Ema::default()],
            motor_rpm: Ema::default(),

            bat_v: Ema::default(),
            bat_cell_v: [Ema::default(), Ema::default(), Ema::default()],
            bat_ii: Ema::default(),
            bat_io: Ema::default(),

            dir_pos: [Ema::default(), Ema::default()],
            dir_bat_v: Ema::default(),
            dir_bat_i: Ema::default(),

            mcb_vi: [Ema::default(), Ema::default()],
            mcb_io: [Ema::default(), Ema::default()],
            mcb_vo: [Ema::default(), Ema::default()],
            mcb_d: [Ema::default(), Ema::default()],

            mcs_bat_avg: Ema::default(),
            mcs_bat_min: Ema::default(),
            mcs_bat_max: Ema::default(),

            mcs_cap_avg: Ema::default(),
            mcs_cap_min: Ema::default(),
            mcs_cap_max: Ema::default(),

            // All “state” + “error” + “last_seen” default to None
            mic_machine_state: None,
            mic_error_code: None,
            mic_last_seen: None,

            mcs_machine_state: None,
            mcs_error_code: None,
            mcs_last_seen: None,

            mam_machine_state: None,
            mam_error_code: None,
            mam_last_seen: None,

            mac_machine_state: None,
            mac_error_code: None,
            mac_last_seen: None,

            mde_machine_state: None,
            mde_error_code: None,
            mde_last_seen: None,

            msc_machine_state: [None, None, None],
            msc_error_code: [None, None, None],
            msc_last_seen: [None, None, None],

            mcb_machine_state: [None, None],
            mcb_error_code: [None, None],
            mcb_last_seen: [None, None],
        }
    }
}

// ----------------------------------------------------------------------------
// 5) TRAIT: “ANY Message that CanUpdate BoatState”
// ----------------------------------------------------------------------------

/// Marker‐trait: any CAN‐deserialized message whose `update(...)` method writes into BoatState.
pub trait BoatStateVariable {
    fn update(message: Self)
    where
        Self: Sized;
}

// ----------------------------------------------------------------------------
// 6) SAMPLE `BoatStateVariable` IMPLEMENTATIONS
//    (you can add more below, one per `modules::<...>::Message` type)
// ----------------------------------------------------------------------------

use crate::can_types::modules;

// ----- Example: MIC motor data message updates `motor_d[0]`, flags, etc. -----

impl BoatStateVariable for modules::mic19::messages::motor::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        // EMA‐filter the new duty‐cycle (0..255) → fraction
        state.motor_d[0].update(100.0 * (message.d as f32) / (u8::MAX as f32));

        state.motor_on = message.motor.motor_on();
        state.dms_on = message.motor.dms_on();
        state.motor_rev = message.motor.reverse();

        // Stamp “last seen” now
        state.mic_last_seen = Some(Instant::now());
    }
}

// ----- Example: MAM motor message updates `motor_d[1]` -------
impl BoatStateVariable for modules::mam19::messages::motor::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.motor_d[1].update((message.duty_cycle as f32) / 100.0);

        state.mam_last_seen = Some(Instant::now());
    }
}

// ----- Example: MIC state message (machine_state + error_code) -------
impl BoatStateVariable for modules::mic19::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mic_machine_state = Some(message.state);
        state.mic_error_code = Some(message.error);
        state.mic_last_seen = Some(Instant::now());
    }
}

// ----- Example: MCS state message -------
impl BoatStateVariable for modules::mcs19::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mcs_machine_state = Some(message.state);
        state.mcs_error_code = Some(message.error);
        state.mcs_last_seen = Some(Instant::now());
    }
}

// ----- MAM state message -------
impl BoatStateVariable for modules::mam19::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mam_machine_state = Some(message.state);
        state.mam_error_code = Some(message.error);
        state.mam_last_seen = Some(Instant::now());
    }
}

// ----- MAC state message -------
impl BoatStateVariable for modules::mac22::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mac_machine_state = Some(message.state);
        state.mac_error_code = Some(message.error);
        state.mac_last_seen = Some(Instant::now());
    }
}

// ----- MDE state message -------
impl BoatStateVariable for modules::mde22::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mde_machine_state = Some(message.state);
        state.mde_error_code = Some(message.error);
        state.mde_last_seen = Some(Instant::now());
    }
}

// ----- MCB (two slots) state messages -------
impl BoatStateVariable for modules::mcb19_1::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mcb_machine_state[0] = Some(message.state);
        state.mcb_error_code[0] = Some(message.error);
        state.mcb_last_seen[0] = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mcb19_2::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mcb_machine_state[1] = Some(message.state);
        state.mcb_error_code[1] = Some(message.error);
        state.mcb_last_seen[1] = Some(Instant::now());
    }
}

// ----- MSC (three slots) state messages -------
impl BoatStateVariable for modules::msc19_1::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.msc_machine_state[0] = Some(message.state);
        state.msc_error_code[0] = Some(message.error);
        state.msc_last_seen[0] = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::msc19_2::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.msc_machine_state[1] = Some(message.state);
        state.msc_error_code[1] = Some(message.error);
        state.msc_last_seen[1] = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::msc19_3::messages::state::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.msc_machine_state[2] = Some(message.state);
        state.msc_error_code[2] = Some(message.error);
        state.msc_last_seen[2] = Some(Instant::now());
    }
}

// ── Example: MSC “ADC” messages update individual `bat_cell_v[i]` ────────────

impl BoatStateVariable for modules::msc19_1::messages::adc::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.bat_cell_v[0].update((message.average as f32) / 100.0);
        // The MSC “state” timestamp was handled by the corresponding state message above
    }
}

impl BoatStateVariable for modules::msc19_2::messages::adc::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.bat_cell_v[1].update((message.average as f32) / 100.0);
    }
}

impl BoatStateVariable for modules::msc19_3::messages::adc::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.bat_cell_v[2].update((message.average as f32) / 100.0);
    }
}

impl BoatStateVariable for modules::msc19_4::messages::adc::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.bat_ii.update((message.average as f32) / 100.0);
    }
}

impl BoatStateVariable for modules::msc19_5::messages::adc::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.bat_io.update((message.average as f32) / 100.0);
    }
}

// ── Other update impls (e.g. pumps, rpm, MDE sensor, etc.) ───────────────────

impl BoatStateVariable for modules::mic19::messages::mde::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.dir_pos[0].update((26.392_962_f32 * ((message.position as f32) / 100.0)) - 135.0);
        state.mic_last_seen = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mic19::messages::pumps::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.pump[0] = message.pumps.pump1();
        state.pump[1] = message.pumps.pump2();
        state.pump[2] = message.pumps.pump3();

        state.mic_last_seen = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mic19::messages::mcs::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.boat_on = message.boat_on.boat_on();
        state.mcs_last_seen = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mt19::messages::rpm::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.motor_rpm.update(message.average as f32);
        state.mcs_last_seen = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mde22::messages::steeringbat_measurements::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.dir_bat_v.update((message.batvoltage as f32) / 100.0);
        state.dir_bat_i.update((message.batcurrent as f32) / 100.0);
        state.dir_pos[1]
            .update((26.392_962_f32 * ((message.tail_position as f32) / 100.0)) - 135.0);

        state.mde_last_seen = Some(Instant::now());
    }
}

impl BoatStateVariable for modules::mcb19_1::messages::measurements::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mcb_d[0].update(100.0 * (message.dt as f32) / (u8::MAX as f32));
        state.mcb_io[0].update((message.output_current as f32) / 100.0);
        state.mcb_vo[0].update((message.output_voltage as f32) / 100.0);
        state.mcb_vi[0].update((message.input_voltage as f32) / 100.0);

        // NOTE: no direct “state” or “error” in a pure “measurements” frame—but
        //       later, when pruning, we will mark stale if no state frame arrives.
    }
}

impl BoatStateVariable for modules::mcb19_2::messages::measurements::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        state.mcb_d[1].update(100.0 * (message.dt as f32) / (u8::MAX as f32));
        state.mcb_io[1].update((message.output_current as f32) / 100.0);
        state.mcb_vo[1].update((message.output_voltage as f32) / 100.0);
        state.mcb_vi[1].update((message.input_voltage as f32) / 100.0);
    }
}

//
// ── NEW: MCS-19 “bat” message handler ──
//
impl BoatStateVariable for modules::mcs19::messages::bat::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        // Convert u16→f32 (divide by 100 to get “volts”)
        let avg_v = (message.average as f32) / 100.0;
        let min_v = (message.min as f32) / 100.0;
        let max_v = (message.max as f32) / 100.0;

        state.mcs_bat_avg.update(avg_v);
        state.mcs_bat_min.update(min_v);
        state.mcs_bat_max.update(max_v);

        // Also update the overall “bat_v” if you want:
        state.bat_v.update(avg_v);
    }
}

//
// ── NEW: MCS-19 “cap” message handler ──
//
impl BoatStateVariable for modules::mcs19::messages::cap::Message {
    fn update(message: Self) {
        let mut state = BOAT_STATE.lock().unwrap();

        // Convert u16→f32 (divide by 100 to get “volts” for capacitor)
        let avg_v = (message.average as f32) / 100.0;
        let min_v = (message.min as f32) / 100.0;
        let max_v = (message.max as f32) / 100.0;

        state.mcs_cap_avg.update(avg_v);
        state.mcs_cap_min.update(min_v);
        state.mcs_cap_max.update(max_v);
    }
}
// ── TODO: add any other `BoatStateVariable` impls for missing CAN messages ────
// (e.g. MCS “bat” messages, etc.)
// ----------------------------------------------------------------------------
