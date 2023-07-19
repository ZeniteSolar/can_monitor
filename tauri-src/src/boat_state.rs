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
    pub pump1: bool,
    pub pump2: bool,
    pub pump3: bool,
    // pub mam state
    // pub MPPTs
    pub motor_d: Ema<10>,
    pub motor_rpm: Ema<10>,
    pub bat_v: Ema<10>,
    pub bat1_v: Ema<10>,
    pub bat2_v: Ema<10>,
    pub bat3_v: Ema<10>,
    pub bat_ii: Ema<10>,
    pub bat_io: Ema<10>,
    pub dir_head_pos: Ema<10>,
    pub dir_bat_v: Ema<10>,
    pub dir_bat_i: Ema<10>,
    pub dir_tail_pos: Ema<10>,
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
