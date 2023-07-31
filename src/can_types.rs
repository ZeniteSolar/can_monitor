#[allow(unused)]
pub mod modules {
    use serde::{Deserialize, Serialize};

    pub trait CanMessageTrait {
        fn signature(&self) -> u8;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", content = "content")]
    pub enum Messages {
        Mic19(mic19::Messages),
    }

    pub mod mic19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 240u8;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(tag = "type", content = "content")]
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod motor {
                /// Mic19 Motor Message ID
                pub const ID: u32 = 31u32;
                // pub const ID: u32 = 9u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Motor controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Motor state flags.
                    pub motor: State,
                    /// Motor Duty Cycle. Units: %
                    pub d: u8,
                    /// Motor Soft Start. Units: %
                    pub i: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                pub struct State {
                    pub motor_on: bool,
                    pub dms_on: bool,
                    pub reverse: bool,
                    #[skip]
                    _unused: specifiers::B5,
                }
            }

            pub mod pumps {
                /// Mic19 Pumps Message ID
                pub const ID: u32 = 41u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub pumps: State,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Pumps state
                pub struct State {
                    pub pump1: bool,
                    pub pump2: bool,
                    pub pump3: bool,
                    #[skip]
                    _unused: specifiers::B5,
                }
            }

            pub mod mppts {
                /// Mic19 MPPTs Message ID
                pub const ID: u32 = 200u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// MPPTs controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub mppts_on: State,
                    /// MPPTs maximum power limitation. Units: %
                    pub pot: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// MPPTs state
                pub struct State {
                    pub mppts_on: bool,
                    #[skip]
                    _unused: specifiers::B7,
                }
            }

            pub mod mcs {
                /// Mic19 MCS Message ID
                pub const ID: u32 = 32u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// MCS controller parameters
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub boat_on: State,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Boat state
                pub struct State {
                    pub boat_on: bool,
                    #[skip]
                    _unused: specifiers::B7,
                }
            }

            pub mod mde {
                /// Mic19 MDE Message ID
                pub const ID: u32 = 33u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Steering wheel controls
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Steering wheel position. Units: °/100
                    pub position: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod steeringbat_measurements {
                /// MDE22 Steeringbat Measurements Message ID
                pub const ID: u32 = 201u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Auxiliar Battery Voltage
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Battery Voltage. Units: V/100
                    pub batvoltage: u16,
                    /// Tail Position. Units: °/100
                    pub tail_position: u16,
                    /// Battery Current. Units: A/100
                    pub batcurrent: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_1 Measurements Message ID
                pub const ID: u32 = 202u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod mcc19_2 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 226u8;

        pub mod messages {

            pub mod state {
                /// MCC19_2 State Message ID
                pub const ID: u32 = 104u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_2 Measurements Message ID
                pub const ID: u32 = 203u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod mcc19_3 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 227u8;

        pub mod messages {

            pub mod state {
                /// MCC19_3 State Message ID
                pub const ID: u32 = 105u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_3 Measurements Message ID
                pub const ID: u32 = 204u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod mcc19_4 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 228u8;

        pub mod messages {

            pub mod state {
                /// MCC19_4 State Message ID
                pub const ID: u32 = 106u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_4 Measurements Message ID
                pub const ID: u32 = 2054u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod mcc19_5 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 229u8;

        pub mod messages {

            pub mod state {
                /// MCC19_5 State Message ID
                pub const ID: u32 = 107u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_5 Measurements Message ID
                pub const ID: u32 = 206u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod mcc19_6 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 239u8;

        pub mod messages {

            pub mod state {
                /// MCC19_6 State Message ID
                pub const ID: u32 = 108u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Module state report
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// State code
                    pub state: u8,
                    pub control: ControlFlags,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Control flags for operating point
                pub struct ControlFlags {
                    pub enable: bool,
                    pub vi_safe_range: bool,
                    pub vo_safe_range: bool,
                    pub vi_stable: bool,
                    pub dt_safe_range: bool,
                    #[skip]
                    _unused: specifiers::B3,
                }
            }

            pub mod measurements {
                /// MCC19_6 Measurements Message ID
                pub const ID: u32 = 207u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    // pub mod mcc19_7 {
    //     use serde::{Deserialize, Serialize};

    //     pub const SIGNATURE: u8 = 239u8;

    //     pub mod messages {

    //         pub mod state {
    //             /// MCC19_7 State Message ID
    //             pub const ID: u32 = 108u32;

    //             use modular_bitfield::{specifiers::*, *};
    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// Module state report
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// State code
    //                 pub state: u8,
    //                 pub control: ControlFlags,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }

    //             #[bitfield(bytes = 1)]
    //             #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
    //             /// Control flags for operating point
    //             pub struct ControlFlags {
    //                 pub enable: bool,
    //                 pub vi_safe_range: bool,
    //                 pub vo_safe_range: bool,
    //                 pub vi_stable: bool,
    //                 pub dt_safe_range: bool,
    //                 #[skip]
    //                 _unused: specifiers::B3,
    //             }
    //         }

    //         pub mod measurements {
    //             /// MCC19_7 Measurements Message ID
    //             pub const ID: u32 = 207u32;

    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// All measurements from the converter
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// Average output voltage. Units: V/100
    //                 pub output_voltage: u16,
    //                 /// Average input current. Units: A/100
    //                 pub input_current: u16,
    //                 /// Average input voltage. Units: V/100
    //                 pub input_voltage: u16,
    //                 /// Converter's duty cycle. Units: %/255
    //                 pub dt: u8,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }
    //         }
    //     }
    // }
    // pub mod mcc19_8 {
    //     use serde::{Deserialize, Serialize};

    //     pub const SIGNATURE: u8 = 239u8;

    //     pub mod messages {

    //         pub mod state {
    //             /// MCC19_8 State Message ID
    //             pub const ID: u32 = 108u32;

    //             use modular_bitfield::{specifiers::*, *};
    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// Module state report
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// State code
    //                 pub state: u8,
    //                 pub control: ControlFlags,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }

    //             #[bitfield(bytes = 1)]
    //             #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
    //             /// Control flags for operating point
    //             pub struct ControlFlags {
    //                 pub enable: bool,
    //                 pub vi_safe_range: bool,
    //                 pub vo_safe_range: bool,
    //                 pub vi_stable: bool,
    //                 pub dt_safe_range: bool,
    //                 #[skip]
    //                 _unused: specifiers::B3,
    //             }
    //         }

    //         pub mod measurements {
    //             /// MCC19_8 Measurements Message ID
    //             pub const ID: u32 = 207u32;

    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// All measurements from the converter
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// Average output voltage. Units: V/100
    //                 pub output_voltage: u16,
    //                 /// Average input current. Units: A/100
    //                 pub input_current: u16,
    //                 /// Average input voltage. Units: V/100
    //                 pub input_voltage: u16,
    //                 /// Converter's duty cycle. Units: %/255
    //                 pub dt: u8,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }
    //         }
    //     }
    // }
    // pub mod mcc19_9 {
    //     use serde::{Deserialize, Serialize};

    //     pub const SIGNATURE: u8 = 239u8;

    //     pub mod messages {

    //         pub mod state {
    //             /// MCC19_9 State Message ID
    //             pub const ID: u32 = 108u32;

    //             use modular_bitfield::{specifiers::*, *};
    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// Module state report
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// State code
    //                 pub state: u8,
    //                 pub control: ControlFlags,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }

    //             #[bitfield(bytes = 1)]
    //             #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
    //             /// Control flags for operating point
    //             pub struct ControlFlags {
    //                 pub enable: bool,
    //                 pub vi_safe_range: bool,
    //                 pub vo_safe_range: bool,
    //                 pub vi_stable: bool,
    //                 pub dt_safe_range: bool,
    //                 #[skip]
    //                 _unused: specifiers::B3,
    //             }
    //         }

    //         pub mod measurements {
    //             /// MCC19_9 Measurements Message ID
    //             pub const ID: u32 = 207u32;

    //             use serde::{Deserialize, Serialize};

    //             #[repr(C, packed)]
    //             #[derive(Debug, Clone, Serialize, Deserialize)]
    //             /// All measurements from the converter
    //             pub struct Message {
    //                 /// Senders signature.
    //                 pub signature: u8,
    //                 /// Average output voltage. Units: V/100
    //                 pub output_voltage: u16,
    //                 /// Average input current. Units: A/100
    //                 pub input_current: u16,
    //                 /// Average input voltage. Units: V/100
    //                 pub input_voltage: u16,
    //                 /// Converter's duty cycle. Units: %/255
    //                 pub dt: u8,
    //             }
    //             impl crate::can_types::modules::CanMessageTrait for Message {
    //                 fn signature(&self) -> u8 {
    //                     self.signature
    //                 }
    //             }
    //         }
    //     }
    // }

    pub mod mab19 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 230u8;

        pub mod messages {

            pub mod state {
                /// MAB19 State Message ID
                pub const ID: u32 = 111u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod pumps {
                /// MAB19 Pumps Message ID
                pub const ID: u32 = 210u32;

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Pumps state
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub pumps: PumpStates,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[bitfield(bytes = 1)]
                #[derive(Debug, Clone, Copy, BitfieldSpecifier, Serialize, Deserialize)]
                /// Pumps state bitfield
                pub struct PumpStates {
                    pub pump1: bool,
                    pub pump2: bool,
                    pub pump3: bool,
                    #[skip]
                    _unused: specifiers::B5,
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod adc {
                /// MSC19_1 ADC Message ID
                pub const ID: u32 = 211u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod msc19_2 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 251u8;
        pub mod messages {

            pub mod state {
                /// MSC19_2 State Message ID
                pub const ID: u32 = 113u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod adc {
                /// MSC19_2 ADC Message ID
                pub const ID: u32 = 212u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod msc19_3 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 252u8;
        pub mod messages {

            pub mod state {
                /// MSC19_3 State Message ID
                pub const ID: u32 = 114u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod adc {
                /// MSC19_3 ADC Message ID
                pub const ID: u32 = 213u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod msc19_4 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 253u8;
        pub mod messages {

            pub mod state {
                /// MSC19_4 State Message ID
                pub const ID: u32 = 115u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod adc {
                /// MSC19_4 ADC Message ID
                pub const ID: u32 = 214u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }

    pub mod msc19_5 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 254u8;
        pub mod messages {

            pub mod state {
                /// MSC19_5 State Message ID
                pub const ID: u32 = 116u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod adc {
                /// MSC19_5 ADC Message ID
                pub const ID: u32 = 215u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Voltage measurements
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod start_stages {
                /// MCS19 Start Stages Message ID
                pub const ID: u32 = 37u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Boat charging // Boat on
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub main_relay: bool,
                    pub charge_relay: bool,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod bat {
                /// MCS19 BAT Message ID
                pub const ID: u32 = 216u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Battery voltage values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod cap {
                /// MCS19 CAP Message ID
                pub const ID: u32 = 217u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Capacitor bank voltage values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                    pub min: u16,
                    pub max: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod rpm {
                /// MT19 RPM Message ID
                pub const ID: u32 = 219u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// RPM motor values
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    pub average: u16,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                use modular_bitfield::{specifiers::*, *};
                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod motor {
                /// MAM19 Motor Message ID
                pub const ID: u32 = 98u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod contactor {
                /// MAM19 Contactor Message ID
                pub const ID: u32 = 36u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Contactor requests
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Control the Contactor State
                    pub request: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }

            pub mod contactor {
                /// MAC22 Contactor Message ID
                pub const ID: u32 = 34u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// Contactor task response
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Contactor task response
                    pub response: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[repr(C, packed)]
                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// All measurements from the converter
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Average output voltage
                    pub output_voltage: u16,
                    /// Average output current
                    pub output_current: u16,
                    /// Average input voltage
                    pub input_voltage: u16,
                    /// Converter's duty cycle
                    pub dt: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }
            }
        }
    }
    pub mod mcb19_2 {
        use serde::{Deserialize, Serialize};

        pub const SIGNATURE: u8 = 221u8;

        pub mod messages {

            pub mod state {
                /// MCB19_2 State Message ID
                pub const ID: u32 = 110u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
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
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
                }

                #[repr(C, packed)]
                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
                /// MCB19_2 Measurements Message ID
                pub const ID: u32 = 209u32;

                use serde::{Deserialize, Serialize};

                #[repr(C, packed)]
                #[derive(Debug, Clone, Serialize, Deserialize)]
                /// All measurements from the converter
                pub struct Message {
                    /// Senders signature.
                    pub signature: u8,
                    /// Average output voltage
                    pub output_voltage: u16,
                    /// Average output current
                    pub output_current: u16,
                    /// Average input voltage
                    pub input_voltage: u16,
                    /// Converter's duty cycle
                    pub dt: u8,
                }
                impl crate::can_types::modules::CanMessageTrait for Message {
                    fn signature(&self) -> u8 {
                        self.signature
                    }
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
