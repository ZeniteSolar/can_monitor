use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum BoatData {
    Modules(Modules),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Modules {
    Mic19(mic19::Data),
}

pub mod mic19 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", content = "content")]
    pub enum Data {
        Motor(motor::Data),
        Pumps(pumps::Data),
        Mppts(mppts::Data),
        Mcs(mcs::Data),
        Mde(mde::Data),
    }

    pub mod motor {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            motor_on: bool,
            dms_on: bool,
            reverse: bool,
            duty_cycle: f32,
            soft_start: f32,
        }

        impl From<crate::can_types::modules::mic19::messages::motor::Message> for Data {
            fn from(value: crate::can_types::modules::mic19::messages::motor::Message) -> Self {
                Self {
                    motor_on: value.motor.motor_on(),
                    dms_on: value.motor.dms_on(),
                    reverse: value.motor.reverse(),
                    duty_cycle: f32::from(value.d / u8::MAX),
                    soft_start: f32::from(value.i / u8::MAX),
                }
            }
        }
    }

    pub mod pumps {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            pump1: bool,
            pump2: bool,
            pump3: bool,
        }
    }

    pub mod mppts {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            mppts_on: bool,
            pot: u8,
        }
    }

    pub mod mcs {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            boat_on: bool,
        }
    }

    pub mod mde {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            position: u16,
        }
    }
}

pub mod mde22 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        SteeringBatMeasurements(steeringbat_measurements::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod steeringbat_measurements {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            battery_voltage: f32,
            tail_position: f32,
            battery_current: f32,
        }
    }
}

pub mod mcc19_1 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Measurements(measurements::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            control: ControlFlags,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ControlFlags {
            enable: bool,
            vi_safe_range: bool,
            vo_safe_range: bool,
            vi_stable: bool,
            dt_safe_range: bool,
        }
    }

    pub mod measurements {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            output_voltage: f32,
            input_current: f32,
            input_voltage: f32,
            duty_cycle: f32,
        }
    }
}

pub mod mab19 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Pumps(pumps::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod pumps {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            pump1: bool,
            pump2: bool,
            pump3: bool,
        }
    }
}

pub mod msc19_1 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Adc(adc::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod adc {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            average: u16,
            min: u16,
            max: u16,
        }
    }
}

pub mod mcs19 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        StartStages(start_stages::Data),
        Bat(bat::Data),
        Cap(cap::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod start_stages {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            main_relay: bool,
            charge_relay: bool,
        }
    }

    pub mod bat {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            average: u16,
            min: u16,
            max: u16,
        }
    }

    pub mod cap {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            average: u16,
            min: u16,
            max: u16,
        }
    }
}

pub mod mt19 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Rpm(rpm::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod rpm {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            average: u16,
        }
    }
}

pub mod mam19 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Motor(motor::Data),
        Contactor(contactor::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod motor {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            duty_cycle: u8,
            soft_start: u8,
        }
    }

    pub mod contactor {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            request: u8,
        }
    }
}

pub mod mac22 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Data {
        State(state::Data),
        Contactor(contactor::Data),
    }

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            state: u8,
            error: u8,
        }
    }

    pub mod contactor {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Data {
            response: u8,
        }
    }
}

pub mod mcb19_1 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub signature: u8,
        pub state: u8,
        pub control: ControlFlags,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ControlFlags {
        pub enable: bool,
        pub vi_safe_range: bool,
        pub vo_safe_range: bool,
        pub vi_stable: bool,
        pub dt_safe_range: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Measurements {
        pub signature: u8,
        pub output_voltage_l: u16,
        pub output_voltage_h: u16,
        pub output_current_l: u16,
        pub output_current_h: u16,
        pub input_voltage_l: u16,
        pub input_voltage_h: u16,
        pub dt: u8,
    }
}
