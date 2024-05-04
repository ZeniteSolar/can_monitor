<template>
  <v-container fill-height fluid class="pa-0 ma-0">
    <v-row no-gutters>

      <!-- LEFT COLUMN -->
      <v-col class="ma-1">

        <MultiMetricCard
          :title="'MPPT'"
          :metricsData="[
          {
            label: 'Vi',
            data: (measurementCards.get('mcc_vi')?.data ?? []) as number[],
            units: (measurementCards.get('mcc_vi')?.units ?? ['']) as string[]
          },
          {
            label: 'II',
            data: (measurementCards.get('mcc_ii')?.data ?? []) as number[],
            units: (measurementCards.get('mcc_ii')?.units ?? ['']) as string[]
          },
          {
            label: 'Vo',
            data: (measurementCards.get('mcc_vo')?.data ?? []) as number[],
            units: (measurementCards.get('mcc_vo')?.units ?? ['']) as string[]
          },
          {
            label: 'Pi',
            data: (measurementCards.get('mcc_pi')?.data ?? []) as number[],
            units: (measurementCards.get('mcc_pi')?.units ?? ['']) as string[]
          },
          {
            label: 'D',
            data: (measurementCards.get('mcc_d')?.data ?? []) as number[],
            units: (measurementCards.get('mcc_d')?.units ?? ['']) as string[]
          },
        ]"
        />
        <MultiMetricCard
          :title="'MCB'"
          :metricsData="[
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
        ]"
        />

        <div v-if="true">
          <MultiStateCard
            :title="'STATE'"
            :stateData="[
              { label: 'MAM', value: measurementCards.get('mam_machine_state')?.data[0] as number },
              { label: 'MIC', value: measurementCards.get('mic_machine_state')?.data[0] as number },
              { label: 'MCS', value: measurementCards.get('mcs_machine_state')?.data[0] as number },
              { label: 'MAC', value: measurementCards.get('mac_machine_state')?.data[0] as number },
              { label: 'MDE', value: measurementCards.get('mde_machine_state')?.data[0] as number },
            ]"
          />
        </div>

      </v-col>

      <!-- p COLUMN -->
      <v-col class="ma-1">
        <MultiMetricCard
            :title="'MAIN BAT'"
            :orientation="Orientation.VERTICAL"
            :metricsData="[
              {
                label: 'BANK',
                data: [
                  measurementCards.get('bat_v')?.avg() ?? 0.0,
                  measurementCards.get('bat_i')?.avg() ?? 0.0,
                  measurementCards.get('bat_p')?.avg() ?? 0.0,
                ],
                units: [
                  measurementCards.get('bat_v')?.units[0] ?? '',
                  measurementCards.get('bat_i')?.units[0] ?? '',
                  measurementCards.get('bat_p')?.units[0] ?? '',
                ]
              },
              {
                label: 'CELL',
                data: (measurementCards.get('bat_cell_v')?.data ?? []) as number[],
                units: (measurementCards.get('bat_cell_v')?.units ?? [] ) as string[],
              },
            ]"
          />

        <MultiMetricCard
          :title = "'MOTOR'"
          :orientation = "Orientation.VERTICAL"
          :metricsData = "[
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
          ]"
        />

      </v-col>

      <!-- RIGHT COLUMN -->
      <v-col class="ma-1">

        <MultiMetricCard
          :title="'STEERING'"
          :metricsData="[
          {
            label: 'D',
            data: (measurementCards.get('dir_pos')?.data ?? []) as number[],
            units: (measurementCards.get('dir_pos')?.units ?? ['']) as string[]
          },
          {
            label: 'B',
            data: [
              measurementCards.get('dir_bat_v')?.avg() ?? 0.0,
              measurementCards.get('dir_bat_i')?.avg() ?? 0.0,
              measurementCards.get('dir_bat_p')?.avg() ?? 0.0,
            ],
            units: [
              measurementCards.get('dir_bat_v')?.units[0] ?? '',
              measurementCards.get('dir_bat_i')?.units[0] ?? '',
              measurementCards.get('dir_bat_p')?.units[0] ?? '',
            ]
          },
        ]"
        />

        <!-- AUXILIAR BATTERIES -->
        <MultiMetricCard
          :title="'AUX BATS'"
          :metricsData="[
          {
            label: 'STE',
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
            label: 'AUX',
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
        ]"
        />

        <!-- SYSTEM CONTROL -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">CONTROL</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- System switches -->
            <v-col class="ma-0 pa-1">
              <v-switch :true-value="true" :model-value="(measurementCards.get('motor_on')!.data[0] as boolean)"
                label="Motor" color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('boat_on')!.data[0] as boolean)"
                label="Boat" color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('dms_on')!.data[0] as boolean)" label="DMS"
                color='red' readonly inset hide-details />

              <v-switch :true-value="true" :model-value="(measurementCards.get('motor_rev')!.data[0] as boolean)"
                label="REV" color='red' readonly inset hide-details />

            </v-col>

            <!-- System switches -->
            <v-col class="ma-0 pa-1">
              <v-switch v-for="(pump, index) in measurementCards.get('pump')!.data" :key="index" :true-value="true"
                :model-value="(pump as boolean)" :label="`BP ${index + 1}`" color='red' readonly inset hide-details />

            </v-col>

          </v-row>

        </v-card>

        <!-- SYSTEM ERRORS -->
        <div v-if="true">
          <v-card class="ma-0 pa-0">
            <v-card-title class="py-0 mt-0 bg-primary font-weight-black">ERRORS</v-card-title>

            <!-- DATA -->

            <!-- MAM -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MAM [{{
                measurementCards.get('mam_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 1" color="green"
                  size="x-small" style="px-2">overcurrent</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 2" color="green"
                  size="x-small" style="px-2">overvoltage</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 4" color="green"
                  size="x-small" style="px-2">overheat</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 8" color="green"
                  size="x-small" style="px-2">fault</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 16" color="green"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mam_error_code')!.data[0] as number) !== 16" color="green"
                  size="x-small" style="px-2">no contactor</v-chip>
              </p>
            </v-row>

            <!-- MIC -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MIC [{{
                measurementCards.get('mic_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mic_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mic_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
              </p>
            </v-row>

            <!-- MCS -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MCS [{{
                measurementCards.get('mcs_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mcs_error_code')!.data[0] as number) !== 2" color="red"
                  size="x-small" style="px-2">no charge</v-chip>
              </p>
            </v-row>

            <!-- MAC -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MAC [{{
                measurementCards.get('mac_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mac_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mac_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
              </p>
            </v-row>

            <!-- MDE -->
            <v-row class="px-2 ma-0 text-mono" style="font-size:10px">
              <p>MDE [{{
                measurementCards.get('mde_error_code')!.data[0] }}]:
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 0" color="green"
                  size="x-small" style="px-2">None</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 1" color="red"
                  size="x-small" style="px-2">no canbus</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 2" color="red"
                  size="x-small" style="px-2">invalid tail</v-chip>
                <v-chip :hidden="(measurementCards.get('mde_error_code')!.data[0] as number) !== 4" color="red"
                  size="x-small" style="px-2">wrong side turn</v-chip>
              </p>
            </v-row>

          </v-card>
        </div>

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
  border-radius: 7px;
}

:deep(.v-switch .v-selection-control) {
  min-height: 10px;
}

:deep(.v-selection-control--density-default) {
  --v-selection-control-size: 20px
}
</style>


<script setup lang="ts">
import { format } from 'numerable'
import { reactive, ref, type Ref } from 'vue'
/* Components */
import MultiMetricCard from './MultiMetricCard.vue';
import MultiStateCard from './MultiStateCard.vue';
/* Types */
import { Orientation } from '@/types/index'
import { GenericCardData } from '../measurement_types'


// Getting data from the backend
// type CanMessage = {
//   timestamp: number,
//   message: string
// }
// const messages: Ref<CanMessage[]> = ref([])
const last_msg_time: Ref<number | null> = ref(null)

const parse_canboat_message = (message: object) => {
  Object.entries(message).forEach((entry) => {
    const [key, data] = entry

    if (data !== null) {
      const card_instance = measurementCards.get(key)
      if (card_instance === null || card_instance === undefined) {
        return
      }

      if (typeof (data) === "number") {
        card_instance.data[0] = data
        return
      }

      if (typeof (data) === "boolean") {
        card_instance.data[0] = data
        return
      }

      if (Array.isArray(data)) {
        card_instance.data = data
        return
      }

    }
  })

  last_msg_time.value = Date.now()
}

const WSAPI = `ws://${window.location.hostname}:3001`
console.log(WSAPI)

class WSConnection {
  socket: WebSocket | null = null

  constructor() { }

  connectWebsocket() {
    this.socket = new WebSocket(WSAPI)

    this.socket.onopen = (event) => {
      console.log("[open] Websocket connection established", event)
    }

    this.socket.onmessage = (event) => {
      let message = JSON.parse(event.data)

      // Get all boat data and update each card accordingly
      parse_canboat_message(message)
    }

    this.socket.onclose = (event) => {
      if (event.wasClean) {
        console.log(
          `[close] Websocket connection closed cleanly, code=${event.code} reason=${event.reason}`
        )
      } else {
        // e.g. server process killed or network down
        // event.code is usually 1006 in this case
        console.log("[close] Websocket connection died")
      }
      setTimeout(this.connectWebsocket, 4000)
    }

    this.socket.onerror = (error) => {
      console.log(`[error] Websocket error: ${JSON.stringify(error)}`)
    }
  }
}

const ws = new WSConnection()
ws.connectWebsocket()

const measurementCards = reactive(new Map<string, GenericCardData>)
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
measurementCards.set("mcc_pi", new GenericCardData(
  "MPPTs Pi",
  "MPPTs Input Power",
  'W',
  0,
  300,
))
measurementCards.set("mcc_vi", new GenericCardData(
  "MPPT Vi",
  "MPPT Input Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcc_ii", new GenericCardData(
  "MPPT Ii",
  "MPPT Input Current",
  'A',
  0,
  15,
))
measurementCards.set("mcc_vo", new GenericCardData(
  "MPPT Vo",
  "MPPT Output Voltage",
  'V',
  0,
  60,
))
measurementCards.set("mcc_d", new GenericCardData(
  "MPPT D",
  "MPPT Duty Cycle",
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