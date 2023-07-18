use serde::{Deserialize, Serialize};

use crate::can_types::modules;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct BoatData {
    motor_d: Option<f32>,
    bat_ii: Option<f32>,
    bat_io: Option<f32>,
}

impl From<modules::mic19::messages::motor::Message> for BoatData {
    fn from(message: modules::mic19::messages::motor::Message) -> Self {
        Self {
            motor_d: Some(100f32 * (message.d as f32) / (u8::MAX as f32)),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_4::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_4::messages::adc::Message) -> Self {
        Self {
            bat_ii: Some(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ..Default::default()
        }
    }
}

impl From<modules::msc19_5::messages::adc::Message> for BoatData {
    fn from(message: modules::msc19_5::messages::adc::Message) -> Self {
        Self {
            bat_io: Some(100f32 * (message.average as f32) / (u16::MAX as f32)),
            ..Default::default()
        }
    }
}
