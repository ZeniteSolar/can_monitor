<template>
  <v-container fill-height fluid class="pa-0 ma-0">
    <v-row no-gutters>

      <!-- LEFT COLUMN -->
      <v-col class="ma-1">

        <!-- MCB - STEERING BATTERY -->
        <MultiMetricCard :title="'MCB - BATERIA DIREÇÃO'" :titleColor="'bg-primary'" :metricsData="[
          {
            label: 'Vi',
            data: (measurementCards.get('mcb_vi')?.data ?? []) as number[],
            units: (measurementCards.get('mcb_vi')?.units ?? ['']) as string[]
          },
          {
            label: 'Io',
            data: (measurementCards.get('mcb_io')?.data ?? []) as number[],
            units: (measurementCards.get('mcb_io')?.units ?? ['']) as string[]
          },
          {
            label: 'Vo',
            data: (measurementCards.get('mcb_vo')?.data ?? []) as number[],
            units: (measurementCards.get('mcb_vo')?.units ?? ['']) as string[]
          },
          {
            label: 'Po',
            data: (measurementCards.get('mcb_po')?.data ?? []) as number[],
            units: (measurementCards.get('mcb_po')?.units ?? ['']) as string[]
          },
        ]" />

        <div v-if="true">
          <!-- MODULES STATE -->
          <MultiStateCard :title="'ESTADO DOS MÓDULOS'" :titleColor="'bg-primary text-white'" :stateData="[
            { label: 'MAM', value: measurementCards.get('mam_machine_state')?.data[0] as number },
            { label: 'MIC', value: measurementCards.get('mic_machine_state')?.data[0] as number },
            { label: 'MCS', value: measurementCards.get('mcs_machine_state')?.data[0] as number },
            { label: 'MAC', value: measurementCards.get('mac_machine_state')?.data[0] as number },
            { label: 'MDE', value: measurementCards.get('mde_machine_state')?.data[0] as number },
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
              measurementCards.get('mcb_vo')?.data[0] as number,
              measurementCards.get('mcb_io')?.data[0] as number,
              measurementCards.get('mcb_po')?.data[0] as number,
            ],
            units: [
              measurementCards.get('mcb_vo')?.units ?? '',
              measurementCards.get('mcb_io')?.units ?? '',
              measurementCards.get('mcb_po')?.units ?? '',
            ]
          },
          {
            label: 'AUXILIAR',
            data: [
              measurementCards.get('mcb_vo')?.data[1] as number,
              measurementCards.get('mcb_io')?.data[1] as number,
              measurementCards.get('mcb_po')?.data[1] as number,
            ],
            units: [
              measurementCards.get('mcb_vo')?.units ?? '',
              measurementCards.get('mcb_io')?.units ?? '',
              measurementCards.get('mcb_po')?.units ?? '',
            ]
          },
        ]" />

        <!-- Main Battery -->
        <MultiMetricCard :title="'BATERIA PRINCIPAL'" :titleColor="'bg-secondary text-black'"
          :orientation="Orientation.VERTICAL" :metricsData="[
            {
              label: 'TOTAL',
              data: (() => {
                const raw = measurementCards.get('bat_cell_v')?.data as unknown[] ?? [];
                const cells = raw
                  .filter((v): v is number => typeof v === 'number')
                  .slice(0, 2);

                if (cells.length === 2) {
                  cells.push((cells[0] + cells[1]) / 2);
                }

                const voltage = cells.reduce((sum: number, v: number) => sum + v, 0);

                return [
                  voltage,
                  measurementCards.get('bat_i')?.avg() ?? 0.0,
                  measurementCards.get('bat_p')?.avg() ?? 0.0,
                ];
              })(),
              units: [
                measurementCards.get('bat_v')?.units?.[0] ?? '',
                measurementCards.get('bat_i')?.units?.[0] ?? '',
                measurementCards.get('bat_p')?.units?.[0] ?? '',
              ]
            },
            {
              label: 'CELULAS',
              data: (() => {
                const raw = measurementCards.get('bat_cell_v')?.data as unknown[] ?? [];
                const safe = raw
                  .filter((v): v is number => typeof v === 'number')
                  .slice(0, 2);

                return safe.length === 2 ? [...safe, (safe[0] + safe[1]) / 2] : safe;
              })(),
              units: (() => {
                const u = measurementCards.get('bat_cell_v')?.units;
                return Array.isArray(u) ? u : u ? [u] : [];
              })()
            },
          ]" />

        <!-- STEERING/ -->
        <SteeringCard :title="'DIREÇÃO'" :titleColor="'bg-secondary text-black'" 
        :angle="steeringAngle" :tailAngle="tailAngle" :metricsData="[
          {
            label: 'B',
            data: [
              measurementCards.get('dir_bat_v')?.avg() ?? 0,
              measurementCards.get('dir_bat_i')?.avg() ?? 0,
              measurementCards.get('dir_bat_p')?.avg() ?? 0
            ],
            units: [
              measurementCards.get('dir_bat_v')?.units[0] ?? '',
              measurementCards.get('dir_bat_i')?.units[0] ?? '',
              measurementCards.get('dir_bat_p')?.units[0] ?? ''
            ]
          }
        ]" />

      </v-col>

      <!-- CONTROL COLUMN -->
      <v-col class="ma-1">

        <!-- MOTOR -->
        <MultiMetricCard :title="'MOTOR'" :titleColor="'bg-terciary text-white'" :orientation="Orientation.VERTICAL"
          :metricsData="[
            {
              label: 'D',
              data: (measurementCards.get('motor_d')?.data ?? []) as number[],
              units: (measurementCards.get('motor_d')?.units ?? ['']) as string[]
            },
            {
              label: 'RPM',
              data: [(measurementCards.get('motor_rpm')?.avg() ?? 0.0)],
              units: [(measurementCards.get('motor_rpm')?.units ?? [''])[0]],
            },
          ]" />

        <!-- CONTROL KEYS -->
        <SwitchDisplay :title="'CONTROLE'" :titleColor="'bg-terciary text-white'" :maxLines="4" :data="[
          { value: measurementCards.get('motor_on')?.data[0] as boolean, label: 'MOTOR' },
          { value: measurementCards.get('boat_on')?.data[0] as boolean, label: 'BOAT' },
          { value: measurementCards.get('dms_on')?.data[0] as boolean, label: 'DMS' },
          { value: measurementCards.get('motor_rev')?.data[0] as boolean, label: 'REV' },
        ].concat(
          (measurementCards.get('pump')?.data as boolean[]).map((value, index) => ({ value, label: `BP ${index + 1}` }))
        )" />

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


<script setup lang="ts">
import { reactive, ref, onUnmounted, type Ref, computed } from 'vue'
import MultiMetricCard from './MultiMetricCard.vue';
import MultiStateCard from './MultiStateCard.vue';
import SwitchDisplay from './SwitchCard.vue';
import SteeringCard from './SteeringCard.vue';
import { Orientation } from '@/types/index'
import { GenericCardData } from '../measurement_types'

const last_msg_time: Ref<number | null> = ref(null)
const measurementCards = reactive(new Map<string, GenericCardData>())

// pull a single angle number out of the array
const steeringAngle = computed(() => {
  const value = measurementCards.get('dir_pos')?.data[0];
  return typeof value === 'number' ? value : 0;
});

const tailAngle = computed(() => {
  const value = measurementCards.get('dir_pos')?.data[1];
  return typeof value === 'number' ? value : 0;
});

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
    await parse_canboat_message(message);
    last_msg_time.value = Date.now();
  }
}

const apiUrl = `ws://${window.location.hostname}:3001`;
const ws = new WSConnection(apiUrl);

async function parse_canboat_message(message: object) {
  for (const [key, data] of Object.entries(message)) {
    const card_instance = measurementCards.get(key);
    if (!card_instance) continue;

    if (typeof data === 'number' || typeof data === 'boolean') {
      card_instance.data[0] = data;
    } else if (Array.isArray(data)) {
      card_instance.data = data;
    }
  }
}

onUnmounted(() => {
  if (ws.socket) {
    ws.socket.close();
  }
});

measurementCards.set("motor_d", new GenericCardData(
  "Motor D",
  "ESC PWM Duty-Cycle",
  '%',
  0,
  100,
))
measurementCards.set("motor_rpm", new GenericCardData(
  "Motor RPM",
  "Motor RPM",
  'RPM',
  0,
  6000,
))
measurementCards.set("bat_v", new GenericCardData(
  "Bat Cell V",
  "Battery Voltage",
  'V',
  30,
  60,
))
measurementCards.set("bat_i", new GenericCardData(
  "Bat I",
  "Battery Current",
  'A',
  -200,
  200,
))
measurementCards.set("bat_p", new GenericCardData(
  "Bat Cell V",
  "Battery Power",
  'W',
  -10000,
  10000,
))
measurementCards.set("bat_cell_v", new GenericCardData(
  "Bat V",
  "Battery Voltage",
  'V',
  10,
  16,
))
measurementCards.set("dir_pos", new GenericCardData(
  "Dir H",
  "Steering System Sensors Position",
  '°',
  -135,
  135,
))
measurementCards.set("dir_bat_v", new GenericCardData(
  "Dir V",
  "Steering System Battery Voltage",
  'V',
  7,
  15,
))
measurementCards.set("dir_bat_i", new GenericCardData(
  "Dir I",
  "Steering System Battery Current",
  'A',
  0,
  20,
))
measurementCards.set("dir_bat_p", new GenericCardData(
  "Dir I",
  "Steering System Battery Power",
  'W',
  0,
  300,
))
measurementCards.set("mcb_po", new GenericCardData(
  "MCBs Po",
  "MCBs Output Power",
  'W',
  0,
  300,
))
measurementCards.set("mcb_vi", new GenericCardData(
  "MCB Vi",
  "MCB Input Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcb_io", new GenericCardData(
  "MCB Io",
  "MCB Output Current",
  'A',
  0,
  15,
))
measurementCards.set("mcb_vo", new GenericCardData(
  "MCB Vo",
  "MCB Output Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcb_d", new GenericCardData(
  "MCB D",
  "MCB Duty Cycle",
  '%',
  0,
  100,
))
measurementCards.set("pump", new GenericCardData(
  "BILDGE PUMP",
  "Bildge Pump Status",
  '',
  0,
  1,
))
measurementCards.set("boat_on", new GenericCardData(
  "BOAT ON",
  "Boat Status",
  '',
  0,
  1,
))
measurementCards.set("dms_on", new GenericCardData(
  "DMS ON",
  "Dead Man Switch Status",
  '',
  0,
  1,
))
measurementCards.set("motor_on", new GenericCardData(
  "MOTOR ON",
  "Motor Status",
  '',
  0,
  1,
))
measurementCards.set("motor_rev", new GenericCardData(
  "MOTOR REVERSE",
  "Motor Reverse Status",
  '',
  0,
  1,
))
measurementCards.set("mam_machine_state", new GenericCardData(
  "MAM MACHINE STATE",
  "MAM Machine State",
  '',
  0,
  1,
))
measurementCards.set("mic_machine_state", new GenericCardData(
  "MIC MACHINE STATE",
  "MIC Machine State",
  '',
  0,
  1,
))
measurementCards.set("mcs_machine_state", new GenericCardData(
  "MCS MACHINE STATE",
  "MCS Machine State",
  '',
  0,
  1,
))
measurementCards.set("mac_machine_state", new GenericCardData(
  "MAC MACHINE STATE",
  "MAC Machine State",
  '',
  0,
  1,
))
measurementCards.set("mde_machine_state", new GenericCardData(
  "MDE MACHINE STATE",
  "MDE Machine State",
  '',
  0,
  1,
))

measurementCards.set("mam_error_code", new GenericCardData(
  "MAM ERROR CODE",
  "MAM Error Code",
  '',
  0,
  1,
))
measurementCards.set("mic_error_code", new GenericCardData(
  "MIC ERROR CODE",
  "MIC Error Code",
  '',
  0,
  1,
))
measurementCards.set("mcs_error_code", new GenericCardData(
  "MCS ERROR CODE",
  "MCS Error Code",
  '',
  0,
  1,
))
measurementCards.set("mac_error_code", new GenericCardData(
  "MAC ERROR CODE",
  "MAC Error Code",
  '',
  0,
  1,
))
measurementCards.set("mde_error_code", new GenericCardData(
  "MDE ERROR CODE",
  "MDE Error Code",
  '',
  0,
  1,
))

</script>