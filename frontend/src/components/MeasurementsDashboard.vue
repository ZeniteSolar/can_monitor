<template>
  <v-container fill-height fluid class="pa-0 ma-0">
    <AppHeader :leftLogo1="'/logo_zenite.png'" :leftLogo2="'/logo_SantaCatarina.png'" :leftLogo3="'/logo_ifsc.png'"
      :rightLogo1="'/logo_lic.png'" :rightLogo2="'/logo_camad.png'" />

    <v-row no-gutters>
      <!-- Your dashboard content continues here -->


      <!-- LEFT COLUMN -->
      <v-col class="ma-1">

        <!-- MCB - STEERING BATTERY -->
        <!-- <MultiMetricCard :title="'MCB - BATERIA DIREÇÃO'" :titleColor="'bg-primary'" :metricsData="[
          {
            label: 'Vi',
            data: (measurementCards['mcb_vi']?.data ?? []) as number[],
            units: [measurementCards['mcb_vi']?.units ?? ''],
          },
          {
            label: 'Io',
            data: (measurementCards['mcb_io']?.data ?? []) as number[],
            units: [measurementCards['mcb_io']?.units ?? ''],
          },
          {
            label: 'Vo',
            data: (measurementCards['mcb_vo']?.data ?? []) as number[],
            units: [measurementCards['mcb_vo']?.units ?? ''],
          },
          {
            label: 'Po',
            data: (measurementCards['mcb_po']?.data ?? []) as number[],
            units: [measurementCards['mcb_po']?.units ?? ''],
          }
        ]" /> -->

        <div v-if="true">
          <!-- MODULES STATE -->
          <MultiStateCard :title="'ESTADO DOS MÓDULOS'" :titleColor="'bg-primary text-white'" :modules="[
            { label: 'MIC', stateKey: 'mic_machine_state', errorKey: 'mic_error_code' },
            { label: 'MCS', stateKey: 'mcs_machine_state', errorKey: 'mcs_error_code' },
            { label: 'MAM', stateKey: 'mam_machine_state', errorKey: 'mam_error_code' },
            { label: 'MAC', stateKey: 'mac_machine_state', errorKey: 'mac_error_code' },
            { label: 'MSC_1', stateKey: 'msc_machine_state', errorKey: 'msc_error_code', index: 0 },
            { label: 'MCB_1', stateKey: 'mcb_machine_state', errorKey: 'mcb_error_code', index: 0 },
            { label: 'MCB_2', stateKey: 'mcb_machine_state', errorKey: 'mcb_error_code', index: 1 },
            { label: 'MDE', stateKey: 'mde_machine_state', errorKey: 'mde_error_code' }
          ]" />
        </div>

      </v-col>

      <!-- p COLUMN -->
      <v-col class="ma-1">

        <!-- AUXILIAR BATTERIES -->
        <MultiMetricCard :title="'BATERIAS AUXILIARES'" :titleColor="'bg-secondary text-black'" :metricsData="[
          {
            label: 'DIREÇÃO',
            data: [
              measurementCards['mcb_vo']?.data[0] as number,
              measurementCards['mcb_io']?.data[0] as number,
              measurementCards['mcb_po']?.data[0] as number,
            ],
            units: [
              measurementCards['mcb_vo']?.units ?? '',
              measurementCards['mcb_io']?.units ?? '',
              measurementCards['mcb_po']?.units ?? '',
            ]
          },
          {
            label: 'AUXILIAR',
            data: [
              measurementCards['mcb_vo']?.data[1] as number,
              measurementCards['mcb_io']?.data[1] as number,
              measurementCards['mcb_po']?.data[1] as number,
            ],
            units: [
              measurementCards['mcb_vo']?.units ?? '',
              measurementCards['mcb_io']?.units ?? '',
              measurementCards['mcb_po']?.units ?? '',
            ]
          }
        ]" />

        <!-- Main Battery -->
        <MultiMetricCard :title="'BATERIA PRINCIPAL'" :titleColor="'bg-secondary text-black'" :metricsData="[
          {
            label: (() => {
              const raw = measurementCards['bat_cell_v']?.data as unknown[] ?? [];
              const cells = raw.filter((v: unknown): v is number => typeof v === 'number');
              return cells.length >= 2 ? 'TOTAL (MSC)' : 'TOTAL (MCB)';
            })(),
            data: (() => {
              const raw = measurementCards['bat_cell_v']?.data as unknown[] ?? [];
              const cells = raw.filter((v: unknown): v is number => typeof v === 'number').slice(0, 2);

              if (cells.length === 2) {
                const avg = (cells[0] + cells[1]) / 2;
                return [
                  cells[0] + cells[1] + avg,
                  measurementCards['bat_i']?.avg() ?? 0,
                  measurementCards['bat_p']?.avg() ?? 0,
                ];
              } else {
                const viRaw = (measurementCards['mcb_vi']?.data as unknown[]) ?? [];
                const validVi = viRaw.filter((v: unknown): v is number => typeof v === 'number');

                const fallback = validVi.length >= 2
                  ? Math.min(validVi[0], validVi[1])
                  : validVi[0] ?? 0;

                return [
                  fallback,
                  measurementCards['bat_i']?.avg() ?? 0,
                  measurementCards['bat_p']?.avg() ?? 0,
                ];
              }
            })(),
            units: [
              measurementCards['bat_v']?.units?.[0] ?? '',
              measurementCards['bat_i']?.units?.[0] ?? '',
              measurementCards['bat_p']?.units?.[0] ?? '',
            ],
          },
          {
            label: 'CELULAS',
            data: (() => {
              const raw = measurementCards['bat_cell_v']?.data as unknown[] ?? [];
              const cells = raw.filter((v: unknown): v is number => typeof v === 'number').slice(0, 2);

              if (cells.length === 2) {
                const avg = (cells[0] + cells[1]) / 2;
                return [...cells, avg];
              } else {
                return [0, 0, 0];
              }
            })(),
            units: (() => {
              const u = measurementCards['bat_cell_v']?.units;
              return Array.isArray(u) ? u : u ? [u, u, u] : ['', '', ''];
            })()
          }
        ]" />



        <!-- STEERING. Use compute methods for visual integrity and separation from metricsData (display only)-->
        <SteeringCard :title="'DIREÇÃO'" :titleColor="'bg-secondary text-black'" :data="[
          measurementCards['dir_pos']?.data?.[0] as number ?? 0,
          measurementCards['dir_pos']?.data?.[1] as number ?? 0
        ]" :metricsData="[
          {
            label: 'B',
            data: [
              measurementCards['dir_bat_v']?.avg() ?? 0,
              measurementCards['dir_bat_i']?.avg() ?? 0,
              measurementCards['dir_bat_p']?.avg() ?? 0,
            ],
            units: [
              measurementCards['dir_bat_v']?.units[0] ?? '',
              measurementCards['dir_bat_i']?.units[0] ?? '',
              measurementCards['dir_bat_p']?.units[0] ?? '',
            ]
          }
        ]" />


      </v-col>

      <!-- TODO review .units logic in this code CONTROL COLUMN -->
      <v-col class="ma-1">
        <!-- CONTROL KEYS -->
        <SwitchDisplay :title="'CONTROLE'" :titleColor="'bg-terciary text-white'" :maxLines="4" :data="[
          { value: measurementCards['boat_on']?.data[0] as boolean, label: 'BOAT' },
          { value: measurementCards['motor_on']?.data[0] as boolean, label: 'MOTOR' },
          { value: measurementCards['motor_rev']?.data[0] as boolean, label: 'REV' },
          { value: measurementCards['dms_on']?.data[0] as boolean, label: 'DMS' },
        ]" />

        <!-- MOTOR -->
        <Speedometer :title="'MOTOR'" :titleColor="'bg-terciary text-white'" :data="[
          measurementCards['motor_d']?.data?.[0] as number ?? 0,
          measurementCards['motor_d']?.data?.[1] as number ?? 0
        ]" />


      </v-col>

    </v-row>

    <v-divider class="my-4"></v-divider>

    <v-card>
      Last message at: {{ last_msg_time }}
    </v-card>

    <!-- <v-card>
      <v-card-title>Can Messages</v-card-title>
      <v-card-item v-for="( message, index ) in  messages.values() " :key="index"> {{ message }}</v-card-item>
    </v-card> -->

  </v-container>
</template>

<style scoped>
.invert {
  mix-blend-mode: difference;
}

/* CUSTOM TINY SWITCH */
:deep(.v-switch__thumb) {
  height: 10px;
  width: 10px;
}

:deep(.v-switch--inset .v-switch__track) {
  height: 14px;
  width: 34px;
}

:deep(.v-switch .v-selection-control) {
  min-height: 10px;
}

:deep(.v-selection-control--density-default) {
  --v-selection-control-size: 20px
}
</style>

<!-- These thresholds are used to determine if the incoming data is significant enough to update the card -->
<!-- For example, if the old value is 10 and the new value is 20, and the threshold is 5, the card will update -->
<!-- If the threshold is 0, it will always update -->
<script lang="ts">

export const thresholds: Record<string, number> = {
  // binary / boolean flags
  boat_on: 0,
  motor_on: 0,
  motor_rev: 0,
  dms_on: 0,

  // small digital flags (array of booleans)
  pump: 0,

  // ESC / motor feedback
  motor_d: 0,
  motor_rpm: 5,

  // machine states (0–4 enums)
  mic_machine_state: 0,
  mcs_machine_state: 0,
  mam_machine_state: 0,
  mac_machine_state: 0,
  msc_machine_state: 0,
  mcb_machine_state: 0,
  mde_machine_state: 0,

  // error codes
  mic_error_code: 0,
  mcs_error_code: 0,
  mam_error_code: 0,
  mac_error_code: 0,
  msc_error_code: 0,
  mcb_error_code: 0,
  mde_error_code: 0,

  // battery block
  bat_v: 0.1,
  bat_cell_v: 0.1,
  bat_ii: 0.1,
  bat_io: 0.1,
  bat_i: 0.1,
  bat_p: 1.5,

  // directional battery
  dir_bat_v: 0.1,
  dir_bat_i: 0.1,
  dir_bat_p: 1,

  // steering / tail angles
  dir_pos: 0,

  // MCB sensors
  mcb_d: 0.01,
  mcb_vi: 0.05,
  mcb_io: 0.05,
  mcb_vo: 0.05,
  mcb_po: 0.5,
};

</script>

<script setup lang="ts">
import { reactive, ref, onUnmounted, type Ref, computed } from 'vue'
import AppHeader from './AppHeader.vue';
import MultiMetricCard from './MultiMetricCard.vue';
import MultiStateCard from './MultiStateCard.vue';
import SwitchDisplay from './SwitchCard.vue';
import SteeringCard from './SteeringCard.vue';
import Speedometer from './Speedometer.vue';
import { Orientation } from '@/types/index'
import type { BoardState } from '@/types/index';
import { GenericCardData } from '../measurement_types'
import { measurementCards } from '@/measurement_cards';

// Computed methods acting as additional safeguards for data access
// This is relevant for angles to avoid 'Nan' values in the SVG

const last_msg_time: Ref<number | null> = ref(null)

class WSConnection {
  socket: WebSocket | null = null;
  latestMessage: string | null = null;
  isProcessing: boolean = false;

  constructor(private readonly apiUrl: string) {
    this.connectWebsocket();
    setInterval(() => {
      if (this.latestMessage && !this.isProcessing) {
        this.isProcessing = true;
        this.processMessage(this.latestMessage).then(() => {
          this.isProcessing = false;
          this.latestMessage = null; // Reset after processing
        });
      }
    }, 100);
  }

  connectWebsocket() {
    this.socket = new WebSocket(this.apiUrl);

    this.socket.onopen = () => console.log("[open] Websocket connection established");
    this.socket.onmessage = (event) => {
      /* Store the latest message */
      this.latestMessage = event.data;
    };
    this.socket.onclose = (event) => {
      console.log(`[close] Websocket connection died, attempting to reconnect...`);
      setTimeout(() => this.connectWebsocket(), 4000);
    };
    this.socket.onerror = (error) => {
      console.log(`[error] Websocket error: ${JSON.stringify(error)}`);
    };
  }

  async processMessage(data: string) {
    const message = JSON.parse(data);
    const now = Date.now();

    Object.entries(message).forEach(([key, val]) => {
      if (
        typeof val === 'number' ||
        typeof val === 'boolean' ||
        (Array.isArray(val) && val.every((v) => typeof v === 'number'))
      ) {
        // Store last update time for this key
        const card = measurementCards[key];
        if (card) card.__touched__ = now;

        updateMetric(key, val);
      } else {
        console.warn(`Invalid data type for key "${key}":`, val);
      }
    });

    last_msg_time.value = now;
  }
}

const apiUrl = `ws://${window.location.hostname}:3001`;
const ws = new WSConnection(apiUrl);

function updateMetric(key: string, data: number[] | number | boolean) {
  const card = measurementCards[key];
  if (!card) return;

  // Normalize incoming to array
  const incoming = Array.isArray(data) ? data : [data];
  if (incoming.length === 0) return;

  const next = incoming[0];
  const old = card.data?.[0];

  // --- Handle boolean logic
  if (typeof next === 'boolean') {
    if (Array.isArray(card.data) && typeof card.data[0] === 'boolean') {
      if (old !== next) {
        card.data = incoming as boolean[];
      }
    } else {
      // If card is newly initialized, accept any valid boolean[]
      card.data = incoming as boolean[];
    }
    return;
  }

  // --- Handle numeric logic
  if (typeof next === 'number') {
    const threshold = thresholds[key] ?? 0;

    if (
      typeof old !== 'number' || // fresh value
      Math.abs(next - old) > threshold ||
      Number.isNaN(old)
    ) {
      if (Array.isArray(card.data) && typeof card.data[0] === 'number') {
        card.data = incoming as number[];
      } else {
        // If card is uninitialized, accept any valid number[]
        card.data = incoming as number[];
      }
    }
  }
}

onUnmounted(() => {
  if (ws.socket) {
    ws.socket.close();
  }
});

// Standard cards
const cardsToRegister: [string, string, string, string, number, number][] = [
  // key,      label,       description,    units, min, max 
  ['motor_d', 'Motor D', 'ESC PWM Duty-Cycle', '%', 0, 100],
  ['motor_rpm', 'Motor RPM', 'Motor RPM', 'RPM', 0, 6000],
  ['bat_v', 'Bat Cell V', 'Battery Voltage', 'V', 30, 60],
  ['bat_i', 'Bat I', 'Battery Current', 'A', -200, 200],
  ['bat_p', 'Bat P', 'Battery Power', 'W', -10000, 10000],
  ['bat_cell_v', 'Bat V', 'Battery Voltage', 'V', 10, 16],
  ['dir_pos', 'Dir H', 'Steering System Sensors Position', '°', -135, 135],
  ['dir_bat_v', 'Dir V', 'Steering System Battery Voltage', 'V', 7, 15],
  ['dir_bat_i', 'Dir I', 'Steering System Battery Current', 'A', 0, 20],
  ['dir_bat_p', 'Dir P', 'Steering System Battery Power', 'W', 0, 300],
  ['mcb_po', 'MCBs Po', 'MCBs Output Power', 'W', 0, 300],
  ['mcb_vi', 'MCB Vi', 'MCB Input Voltage', 'V', 0, 60],
  ['mcb_io', 'MCB Io', 'MCB Output Current', 'A', 0, 15],
  ['mcb_vo', 'MCB Vo', 'MCB Output Voltage', 'V', 0, 60],
  ['mcb_d', 'MCB D', 'MCB Duty Cycle', '%', 0, 100],
  ['boat_on', 'BOAT ON', 'Boat Status', '', 0, 1],
  ['dms_on', 'DMS ON', 'Dead Man Switch Status', '', 0, 1],
  ['motor_on', 'MOTOR ON', 'Motor Status', '', 0, 1],
  ['motor_rev', 'MOTOR REVERSE', 'Motor Reverse Status', '', 0, 1],
  ['mic_machine_state', 'MIC MACHINE STATE', 'MIC Machine State', '', 0, 4],
  ['mcs_machine_state', 'MCS MACHINE STATE', 'MCS Machine State', '', 0, 4],
  ['mam_machine_state', 'MAM MACHINE STATE', 'MAM Machine State', '', 0, 4],
  ['mac_machine_state', 'MAC MACHINE STATE', 'MAC Machine State', '', 0, 4],
  ['msc_machine_state', 'MSC MACHINE STATE', 'MSC Machine State', '', 0, 4],
  ['mcb_machine_state', 'MCB MACHINE STATE', 'MCB Machine State', '', 0, 4],
  ['mde_machine_state', 'MDE MACHINE STATE', 'MDE Machine State', '', 0, 4],
  ['mic_error_code', 'MIC ERROR CODE', 'MIC Error Code', '', 0, 1],
  ['mcs_error_code', 'MCS ERROR CODE', 'MCS Error Code', '', 0, 1],
  ['mam_error_code', 'MAM ERROR CODE', 'MAM Error Code', '', 0, 1],
  ['mac_error_code', 'MAC ERROR CODE', 'MAC Error Code', '', 0, 1],
  ['msc_error_code', 'MSC ERROR CODE', 'MSC Error Code', '', 0, 1],
  ['mcb_error_code', 'MCB ERROR CODE', 'MCB Error Code', '', 0, 1],
  ['mde_error_code', 'MDE ERROR CODE', 'MDE Error Code', '', 0, 1],
];

cardsToRegister.forEach(([key, label, desc, units, min, max]) => {
  measurementCards[key] = new GenericCardData(label, desc, units, min, max);
});

</script>
