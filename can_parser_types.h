// CODE GENERATED USING MAKOTEMPLATES.ORG, DO NOT EDIT.
#define CAN_VERSION "0.1.2"

#include <stdint.h>


// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_generic_state_msg_t;

// Generic topic
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
        };
    };
} can_generic_generic_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mic19_state_msg_t;

// Motor controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Motor state
                uint8_t motor_on : 1;
                uint8_t dms_on : 1;
                uint8_t reverse : 1;
                uint8_t _unused : 5;
            } motor;
            uint8_t d;  // Motor Duty Cycle. Units: %
            uint8_t i;  // Motor Soft Start. Units: %
        };
    };
} can_mic19_motor_msg_t;

// Pumps controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Pumps state
                uint8_t pump1 : 1;
                uint8_t pump2 : 1;
                uint8_t pump3 : 1;
                uint8_t _unused : 5;
            } pumps;
        };
    };
} can_mic19_pumps_msg_t;

// Mppts controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // MPPTs state
                uint8_t mppts_on : 1;
                uint8_t _unused : 7;
            } mppts_on;
            uint8_t pot;  // MPPTs maximum power limitation. Units: %
        };
    };
} can_mic19_mppts_msg_t;

// MCS controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Boat state
                uint8_t boat_on : 1;
                uint8_t _unused : 7;
            } boat_on;
        };
    };
} can_mic19_mcs_msg_t;

// Steereing wheel controls
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Steering wheel position, byte HIGH. Units: °/100
                uint16_t position;
                struct {
                    uint8_t position_l;
                    uint8_t position_h;
                };
            };
        };
    };
} can_mic19_mde_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mde22_state_msg_t;

// Auxiliar Battery Voltage
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Battery Voltage, bytes low/high. Units: V/100
                uint16_t batvoltage;
                struct {
                    uint8_t batvoltage_l;
                    uint8_t batvoltage_h;
                };
            };
            union {  // Tail Poisition, bytes low/high. Units: °/100
                uint16_t position;
                struct {
                    uint8_t position_l;
                    uint8_t position_h;
                };
            };
            union {  // Battery Current, bytes low/high. Units: A/100
                uint16_t batcurrent;
                struct {
                    uint8_t batcurrent_l;
                    uint8_t batcurrent_h;
                };
            };
        };
    };
} can_mde22_steeringbat_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mvc19_1_state_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mvc19_2_state_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_1_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_1_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_2_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_2_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_3_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_3_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_4_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_4_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_5_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_5_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcc19_6_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average input current, bytes low/high. Units: A/100
                uint16_t input_current;
                struct {
                    uint8_t input_current_l;
                    uint8_t input_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcc19_6_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcb19_1_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average output current, bytes low/high. Units: A/100
                uint16_t output_current;
                struct {
                    uint8_t output_current_l;
                    uint8_t output_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcb19_1_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            struct { // Control flags for operating point
                uint8_t enable : 1;
                uint8_t vi_safe_range : 1;
                uint8_t vo_safe_range : 1;
                uint8_t vi_stable : 1;
                uint8_t dt_safe_range : 1;
                uint8_t _unused : 3;
            } control;
        };
    };
} can_mcb19_2_state_msg_t;

// All measurements from the converter
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average output voltage, bytes low/high. Units: V/100
                uint16_t output_voltage;
                struct {
                    uint8_t output_voltage_l;
                    uint8_t output_voltage_h;
                };
            };
            union {  // Average output current, bytes low/high. Units: A/100
                uint16_t output_current;
                struct {
                    uint8_t output_current_l;
                    uint8_t output_current_h;
                };
            };
            union {  // Average input voltage, bytes low/high. Units: V/100
                uint16_t input_voltage;
                struct {
                    uint8_t input_voltage_l;
                    uint8_t input_voltage_h;
                };
            };
            uint8_t dt;  // converter's duty cycle. Units: %/255
        };
    };
} can_mcb19_2_measurements_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mac22_state_msg_t;

// Contactor task response
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t response;  // Contactor task response. Units: 
        };
    };
} can_mac22_contactor_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mam19_state_msg_t;

// Motor controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t d;  // Motor Duty Cycle. Units: %
            uint8_t i;  // Motor Soft Start. Units: %
        };
    };
} can_mam19_motor_msg_t;

// Contactor requests
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t request;  // Control the Contactor State. Units: 
        };
    };
} can_mam19_contactor_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mab19_state_msg_t;

// Pumps state
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Pumps state
                uint8_t pump1 : 1;
                uint8_t pump2 : 1;
                uint8_t pump3 : 1;
                uint8_t _unused : 5;
            } pumps;
        };
    };
} can_mab19_pumps_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_msc19_1_state_msg_t;

// Voltage measurements
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: V/100
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: V/100
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: V/100
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_msc19_1_adc_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_msc19_2_state_msg_t;

// Voltage measurements
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: V/100
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: V/100
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: V/100
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_msc19_2_adc_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_msc19_3_state_msg_t;

// Voltage measurements
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: V/100
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: V/100
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: V/100
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_msc19_3_adc_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_msc19_4_state_msg_t;

// Current measurements
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: A/100
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: A/100
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: A/100
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_msc19_4_adc_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_msc19_5_state_msg_t;

// Current measurements
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: A/100
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: A/100
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: A/100
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_msc19_5_adc_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mcs19_state_msg_t;

// Boat charging // Boat on
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Boat on/off
                uint8_t main_relay : 1;
                uint8_t _unused : 7;
            } main_relay;
            struct { // Boat Charging
                uint8_t charge_relay : 1;
                uint8_t _unused : 7;
            } charge_relay;
        };
    };
} can_mcs19_start_stages_msg_t;

// battery voltage values
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: 
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: 
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: 
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_mcs19_bat_msg_t;

// capacitor bank voltage values
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // Average, bytes low/high. Units: 
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
            union {  // Min bytes low/high. Units: 
                uint16_t min;
                struct {
                    uint8_t min_l;
                    uint8_t min_h;
                };
            };
            union {  // Max byte low. Units: 
                uint16_t max;
                struct {
                    uint8_t max_l;
                    uint8_t max_h;
                };
            };
        };
    };
} can_mcs19_cap_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mt19_state_msg_t;

// RPM motor values
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            union {  // RPM Average, bytes low/high. Units: 
                uint16_t avg;
                struct {
                    uint8_t avg_l;
                    uint8_t avg_h;
                };
            };
        };
    };
} can_mt19_rpm_msg_t;

// Module state report
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            uint8_t state;  // State code. Units: 
            uint8_t error;  // Error code. Units: 
        };
    };
} can_mswi19_state_msg_t;

// Motor controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Motor state
                uint8_t motor_on : 1;
                uint8_t dms_on : 1;
                uint8_t _unused : 6;
            } motor;
            uint8_t d;  // Motor Duty Cycle. Units: %
            uint8_t i;  // Motor Soft Start. Units: %
        };
    };
} can_mswi19_motor_msg_t;

// Pumps controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Pumps state
                uint8_t pump1 : 1;
                uint8_t pump2 : 1;
                uint8_t pump3 : 1;
                uint8_t _unused : 5;
            } pumps;
        };
    };
} can_mswi19_pumps_msg_t;

// Mppts controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // MPPTs state
                uint8_t mppts_on : 1;
                uint8_t _unused : 7;
            } mppts_on;
            uint8_t pot;  // MPPTs maximum power limitation. Units: %
        };
    };
} can_mswi19_mppts_msg_t;

// MCS controller parameters
typedef struct
{
    union {
        uint8_t raw[8];
        struct {
            uint8_t signature;  // Senders signature. Units: 
            struct { // Boat state
                uint8_t boat_on : 1;
                uint8_t _unused : 7;
            } boat_on;
        };
    };
} can_mswi19_mcs_msg_t;
