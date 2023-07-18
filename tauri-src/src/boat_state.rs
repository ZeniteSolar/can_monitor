use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BOAT_STATE: Arc<Mutex<BoatState>> = Default::default();
}

#[derive(Default, Debug)]
pub struct BoatState {
    pub boat_on: bool,
    pub motor_on: bool,
    // TODO: mam state
    // TODO: MPPTs
    pub motor_d: Ema<100>,
    pub motor_rpm: Ema<100>,
    pub bat_v: Ema<100>,
    pub bat1_v: Ema<100>,
    pub bat2_v: Ema<100>,
    pub bat3_v: Ema<100>,
    pub bat_ii: Ema<100>,
    pub bat_io: Ema<100>,
    pub dir_head_pos: Ema<100>,
    pub dir_bat_v: Ema<100>,
    pub dir_bat_i: Ema<100>,
    pub dir_tail_pos: Ema<100>,
    pub pump1: bool,
    pub pump2: bool,
    pub pump3: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Ema<const MAX_SAMPLES: usize> {
    samples: usize,
    sum: f32,
}

impl<const MAX_SAMPLES: usize> Ema<MAX_SAMPLES> {
    pub fn update(&mut self, value: f32) -> f32 {
        if self.samples < MAX_SAMPLES {
            self.samples += 1;
        }

        self.sum += (value - self.sum) / (self.samples as f32);
        self.sum
    }
}
