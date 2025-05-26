use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::can_types::modules::CanMessageTrait;

lazy_static! {
    pub static ref BOAT_STATE: Arc<Mutex<BoatState>> = Default::default();
}

#[derive(Default, Debug, Clone)]
pub struct BoatState {
    pub boat_on: bool,
    pub motor_on: bool,
    pub motor_rev: bool,
    pub dms_on: bool,
    pub pump: [bool; 3],
    pub motor_d: [Ema<10>; 2],
    pub motor_rpm: Ema<10>,
    pub mic_machine_state: u8,
    pub mcs_machine_state: u8,
    pub mam_machine_state: u8,
    pub mac_machine_state: u8,
    pub msc_machine_state: [u8; 3],
    pub mcb_machine_state: [u8; 2],
    pub mde_machine_state: u8,
    pub mic_error_code: u8,
    pub mcs_error_code: u8,
    pub mam_error_code: u8,
    pub mac_error_code: u8,
    pub msc_error_code: [u8; 3],
    pub mcb_error_code: [u8; 2],
    pub mde_error_code: u8,
    // pub mcb_control: [u8; 2],
    pub bat_v: Ema<10>,
    pub bat_cell_v: [Ema<10>; 3],
    pub bat_ii: Ema<10>,
    pub bat_io: Ema<10>,
    pub dir_pos: [Ema<10>; 2],
    pub dir_bat_v: Ema<10>,
    pub dir_bat_i: Ema<10>,
    pub mcb_vo: [Ema<10>; 2],
    pub mcb_io: [Ema<10>; 2],
    pub mcb_vi: [Ema<10>; 2],
    pub mcb_d: [Ema<10>; 2],
}

#[derive(Default, Debug, Clone)]
pub struct Ema<const MAX_SAMPLES: usize> {
    samples: usize,
    sum: f32,
}

impl<const MAX_SAMPLES: usize> Ema<MAX_SAMPLES> {
    pub fn update(&mut self, value: f32) {
        if self.samples < MAX_SAMPLES {
            self.samples += 1;
        }

        self.sum += (value - self.sum) / (self.samples as f32);
    }
    pub fn value(self) -> f32 {
        self.sum
    }
}

pub trait BoatStateVariable {
    fn update(message: Self)
    where
        Self: CanMessageTrait;
}
