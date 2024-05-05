interface Wrappedf32Vec {
    values: number[];
    values_pct: number[];
    sum: number;
    sum_pct: number;
    avg: number;
    avg_pct: number;
    units: string;
}

interface Wrappedf32 {
    value: number;
    value_pct: number;
    units: string;
}

interface BoatData {
    boat_on: boolean;
    motor_on: boolean;
    motor_rev: boolean;
    dms_on: boolean;
    pump: [boolean, boolean, boolean];
    motor_d: Wrappedf32Vec;
    motor_rpm: Wrappedf32;
    mam_machine_state: number;
    mic_machine_state: number;
    mcs_machine_state: number;
    mac_machine_state: number;
    mde_machine_state: number;
    mam_error_code: number;
    mic_error_code: number;
    mcs_error_code: number;
    mac_error_code: number;
    mde_error_code: number;
    bat_v: Wrappedf32;
    bat_cell_v: Wrappedf32Vec;
    bat_ii: Wrappedf32;
    bat_io: Wrappedf32;
    bat_i: Wrappedf32;
    bat_p: Wrappedf32;
    dir_bat_v: Wrappedf32;
    dir_bat_i: Wrappedf32;
    dir_bat_p: Wrappedf32;
    dir_pos: Wrappedf32Vec;
    mcb_d: Wrappedf32Vec;
    mcb_vi: Wrappedf32Vec;
    mcb_io: Wrappedf32Vec;
    mcb_vo: Wrappedf32Vec;
    mcb_po: Wrappedf32Vec;
    mcc_d: Wrappedf32Vec;
    mcc_ii: Wrappedf32Vec;
    mcc_vi: Wrappedf32Vec;
    mcc_vo: Wrappedf32Vec;
    mcc_pi: Wrappedf32Vec;
}