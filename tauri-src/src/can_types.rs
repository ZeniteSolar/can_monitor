#[allow(unused)]
pub mod modules {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Messages {
        Mic19(mic19::Messages),
    }

    pub mod mic19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 240u8;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Messages {
            State(messages::state::Message),
            Motor(messages::motor::Message),
            Pumps(messages::pumps::Message),
            Mppts(messages::mppts::Message),
            Mcs(messages::mcs::Message),
            Mde(messages::mde::Message),
        }

        pub mod messages {

            pub mod state {
                /// MIC19 State Message ID
                pub const ID: u32 = 30u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code.
                    pub state: u8,
                    /// Error code.
                    pub error: u8,
                }
            }

            pub mod motor {
                /// Mic19 Motor Message ID
                // pub const ID: u32 = 31u32;
                pub const ID: u32 = 9u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Motor controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub motor: u8,
                    /// Motor Duty Cycle. Units: %
                    pub d: u8,
                    /// Motor Soft Start. Units: %
                    pub i: u8,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Motor state
                pub struct State {
                    pub motor_on: bool,
                    pub dms_on: bool,
                    pub reverse: bool,
                }

                bitfield! {
                    struct StateBits(u8);
                    impl Debug;
                    motor_on, set_motor_on: 0;
                    dms_on, set_dms_on: 1;
                    reverse, set_reverse: 2;
                }
            }

            pub mod pumps {
                /// Mic19 Pumps Message ID
                pub const ID: u32 = 41u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub pumps: State,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps state
                pub struct State {
                    pub pump1: bool,
                    pub pump2: bool,
                    pub pump3: bool,
                }

                bitfield! {
                    struct StateBits(u8);
                    impl Debug;
                    pump1, set_pump1: 0;
                    pump2, set_pump2: 1;
                    pump3, set_pump3: 2;
                }
            }

            pub mod mppts {
                /// Mic19 MPPTs Message ID
                pub const ID: u32 = 200u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// MPPTs controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub mppts_on: State,
                    /// MPPTs maximum power limitation. Units: %
                    pub pot: u8,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// MPPTs state
                pub struct State {
                    pub mppts_on: bool,
                }

                bitfield! {
                    struct StateBits(u8);
                    impl Debug;
                    mppts_on, set_mppts_on: 0;
                }
            }

            pub mod mcs {
                /// Mic19 MCS Message ID
                pub const ID: u32 = 32u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// MCS controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub boat_on: State,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Boat state
                pub struct State {
                    pub boat_on: bool,
                }

                bitfield! {
                    struct StateBits(u8);
                    impl Debug;
                    boat_on, set_boat_on: 0;
                }
            }

            pub mod mde {
                /// Mic19 MDE Message ID
                pub const ID: u32 = 33u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Steering wheel controls
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Steering wheel position, byte HIGH. Units: °/100
                    pub position: u16,
                }
            }
        }
    }

    pub mod mde22 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 170u8;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Messages {
            State(messages::state::Message),
            SteeringBatMeasurements(messages::steeringbat_measurements::Message),
        }

        pub mod messages {

            pub mod state {
                /// MDE22 State Message ID
                pub const ID: u32 = 100u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code.
                    pub state: u8,
                    /// Error code.
                    pub error: u8,
                }
            }

            pub mod steeringbat_measurements {
                /// MDE22 Steeringbat Measurements Message ID
                pub const ID: u32 = 201u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Auxiliar Battery Voltage
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Battery Voltage, byte low. Units: V/100
                    pub batvoltage: u16,
                    /// Tail Position, byte low. Units: °/100
                    pub tail_position: u16,
                    /// Battery Current, byte low. Units: A/100
                    pub batcurrent: u16,
                }
            }
        }
    }

    pub mod mcc19_1 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 225u8;

        pub mod messages {

            pub mod state {
                /// MCC19_1 State Message ID
                pub const ID: u32 = 103u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                }

                bitfield! {
                    struct ControlBits(u8);
                    impl Debug;
                    enable, set_enable: 0;
                    vi_safe_range, set_vi_safe_range: 1;
                    vo_safe_range, set_vo_safe_range: 2;
                    vi_stable, set_vi_stable: 3;
                    dt_safe_range, set_dt_safe_range: 4;
                }
            }

            pub mod measurements {
                /// MCC19_1 Measurements Message ID
                pub const ID: u32 = 202u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// All measurements from the converter
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Average output voltage. Units: V/100
                    pub output_voltage: u16,
                    /// Average input current. Units: A/100
                    pub input_current: u16,
                    /// Average input voltage. Units: V/100
                    pub input_voltage: u16,
                    /// Converter's duty cycle. Units: %/255
                    pub dt: u8,
                }
            }
        }
    }

    pub mod mab19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 230u8;

        pub mod messages {

            pub mod state {
                /// MAB19 State Message ID
                pub const ID: u32 = 111u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod pumps {
                /// MAB19 Pumps Message ID
                pub const ID: u32 = 210u32;

                use bitfield::bitfield;
                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps state
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub pumps: PumpStates,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps state bitfield
                pub struct PumpStates {
                    pub pump1: bool,
                    pub pump2: bool,
                    pub pump3: bool,
                }

                bitfield! {
                    struct PumpBits(u8);
                    impl Debug;
                    pump1, set_pump1: 0;
                    pump2, set_pump2: 1;
                    pump3, set_pump3: 2;
                }
            }
        }
    }

    pub mod msc19_1 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 250u8;

        pub mod messages {

            pub mod state {
                /// MSC19_1 State Message ID
                pub const ID: u32 = 112u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod adc {
                /// MSC19_1 ADC Message ID
                pub const ID: u32 = 211u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
            }
        }
    }

    pub mod mcs19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 200u8;

        pub mod messages {

            pub mod state {
                /// MCS19 State Message ID
                pub const ID: u32 = 117u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod start_stages {
                /// MCS19 Start Stages Message ID
                pub const ID: u32 = 37u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Boat charging // Boat on
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub main_relay: bool,
                    pub charge_relay: bool,
                }
            }

            pub mod bat {
                /// MCS19 BAT Message ID
                pub const ID: u32 = 216u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Battery voltage values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
            }

            pub mod cap {
                /// MCS19 CAP Message ID
                pub const ID: u32 = 217u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Capacitor bank voltage values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
            }
        }
    }

    pub mod mt19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 255u8;

        pub mod messages {

            pub mod state {
                /// MT19 State Message ID
                pub const ID: u32 = 218u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod rpm {
                /// MT19 RPM Message ID
                pub const ID: u32 = 219u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// RPM motor values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                }
            }
        }
    }

    pub mod mam19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 190u8;

        pub mod messages {

            pub mod state {
                /// MAM19 State Message ID
                pub const ID: u32 = 99u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod motor {
                /// MAM19 Motor Message ID
                pub const ID: u32 = 98u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Motor controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Motor Duty Cycle
                    pub duty_cycle: u8,
                    /// Motor Soft Start
                    pub soft_start: u8,
                }
            }

            pub mod contactor {
                /// MAM19 Contactor Message ID
                pub const ID: u32 = 36u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Contactor requests
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Control the Contactor State
                    pub request: u8,
                }
            }
        }
    }

    pub mod mac22 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 180u8;

        pub mod messages {

            pub mod state {
                /// MAC22 State Message ID
                pub const ID: u32 = 35u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Error code
                    pub error: u8,
                }
            }

            pub mod contactor {
                /// MAC22 Contactor Message ID
                pub const ID: u32 = 34u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Contactor task response
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Contactor task response
                    pub response: u8,
                }
            }
        }
    }

    pub mod mcb19_1 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 220u8;

        pub mod messages {

            pub mod state {
                /// MCB19_1 State Message ID
                pub const ID: u32 = 109u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    /// Control flags for operating point
                    pub control: ControlFlags,
                }

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                }
            }

            pub mod measurements {
                /// MCB19_1 Measurements Message ID
                pub const ID: u32 = 208u32;

                use serde::{Deserialize, Serialize};

                #[repr(C)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// All measurements from the converter
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Average output voltage, byte low
                    pub output_voltage_l: u16,
                    /// Average output voltage, byte high
                    pub output_voltage_h: u16,
                    /// Average output current, byte low
                    pub output_current_l: u16,
                    /// Average output current, byte high
                    pub output_current_h: u16,
                    /// Average input voltage, byte low
                    pub input_voltage_l: u16,
                    /// Average input voltage, byte high
                    pub input_voltage_h: u16,
                    /// Converter's duty cycle
                    pub dt: u8,
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::can_types::*;

//     #[test]
//     fn basic_test() {
//         let msg = modules::mic19::messages::motor::Message {
//             signature: modules::mic19::SIGNATURE,
//             motor: modules::mic19::messages::motor::State {
//                 motor_on: true,
//                 dms_on: true,
//                 reverse: false,
//             },
//             d: 128,
//             i: 0,
//         };

//         println!("{msg:#?}");
//     }
// }
