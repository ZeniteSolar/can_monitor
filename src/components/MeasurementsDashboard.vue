<template>
  <v-container fill-height fluid class="pa-0 ma-0">
    <v-row no-gutters>

      <!-- LEFT COLUMN -->
      <v-col class="ma-1">

        <!-- MPPT -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MPPT</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- MPPT Vi -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vi
              </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_vi')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_vi')!.avg(), '00.0') }} {{ measurementCards.get('mcc_vi')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_vi')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_vi')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_vi')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Ii -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Ii </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_ii')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_ii')!.avg(), '00.0') }} {{ measurementCards.get('mcc_ii')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_ii')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_ii')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_ii')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Duty Cycle -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> D </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_d')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_d')!.avg(), '00.0') }} {{ measurementCards.get('mcc_d')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_d')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_d')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_d')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Vo -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Vo </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_vo')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_vo')!.avg(), '00.0') }} {{ measurementCards.get('mcc_vo')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_vo')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_vo')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('mcc_vo')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- MPPT Pi -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Pi </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('mcc_pi')!.sum(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('mcc_pi')!.sum(), '0000') }} {{ measurementCards.get('mcc_pi')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('mcc_pi')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('mcc_pi')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '000') }} {{ measurementCards.get('mcc_pi')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

          </v-row>

        </v-card>

        <!-- SYSTEM STATE -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">STATE</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- MAM -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> MAM </v-card-text>
              <v-radio-group :model-value="String(measurementCards.get('mam_machine_state')!.data[0] as number)">
                <v-radio style="font-size: 12px;" class="text-caption" label="INITIALIZING" color="orange" value="0" />
                <v-radio style="font-size: 12px;" class="text-caption" label="CONTACTOR" color="orange" value="1" />
                <v-radio style="font-size: 12px;" class="text-caption" label="IDLE" color="orange" value="2" />
                <v-radio style="font-size: 12px;" class="text-caption" label="RUNNING" color="green" value="3" />
                <v-radio style="font-size: 12px;" class="text-caption" label="ERROR" color="red" value="4" />
              </v-radio-group>
            </v-col>

          </v-row>

        </v-card>

      </v-col>

      <!-- p COLUMN -->
      <v-col class="ma-1">

        <!-- BATTERY -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MAIN BAT</v-card-title>

          <!-- DATA -->

          <!-- BATTERY V -->
          <v-row class="px-2 ma-0">
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Bank </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('bat_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_v')!.avg(), '00.00') }} {{
                      measurementCards.get('bat_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('bat_i')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_i')!.avg(), '000.0') }} {{
                      measurementCards.get('bat_i')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('bat_p')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_p')!.avg(), '00000.0') }} {{
                      measurementCards.get('bat_p')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>
          </v-row>

          <!-- BATTERY CELL -->
          <v-row class="px-2 ma-0">
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> Cell </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('bat_cell_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('bat_cell_v')!.avg(), '00.000') }} {{
                      measurementCards.get('bat_cell_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <div v-for="(value, index) in measurementCards.get('bat_cell_v')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('bat_cell_v')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.000') }} {{ measurementCards.get('bat_cell_v')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>

            </v-col>
          </v-row>

        </v-card>

        <!-- MOTOR -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">MOTOR</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Motor -->
            <v-col class="ma-0 pa-1">
              <div v-for="(value, index) in measurementCards.get('motor_d')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('motor_d')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('motor_d')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>

              <v-progress-linear :model-value="measurementCards.get('motor_rpm')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('motor_rpm')!.avg(), '0000.0') }} {{
                      measurementCards.get('motor_rpm')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

          </v-row>

        </v-card>

      </v-col>

      <!-- RIGHT COLUMN -->
      <v-col class="ma-1">

        <!-- STEERING -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">STEERING</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Steering Direction -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> D </v-card-text>

              <div v-for="(value, index) in measurementCards.get('dir_pos')?.data" :key="index">
                <v-progress-linear :model-value="measurementCards.get('dir_pos')!.item(true, index)" height="11px"
                  class="pa-0 my-1">
                  <template v-slot:default="">
                    <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                      {{ format(value as number, '00.0') }} {{ measurementCards.get('dir_pos')!.units }}
                    </p>
                  </template>
                </v-progress-linear>
              </div>
            </v-col>

            <!-- Steering Battery -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> B </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_v')!.avg(), '00.00') }} {{
                      measurementCards.get('dir_bat_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_i')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_i')!.avg(), '00.00') }} {{
                      measurementCards.get('dir_bat_i')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('dir_bat_p')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('dir_bat_p')!.avg(), '000.0') }} {{
                      measurementCards.get('dir_bat_p')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

            </v-col>

          </v-row>

        </v-card>

        <!-- AUXILIAR BATTERY -->
        <v-card class="ma-0 pa-0">
          <v-card-title class="py-0 mt-0 bg-primary font-weight-black">AUX BAT</v-card-title>

          <!-- DATA -->
          <v-row class="px-2 ma-0">

            <!-- Auxiliar Battery -->
            <v-col class="ma-0 pa-1">
              <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0"> B </v-card-text>

              <v-progress-linear :model-value="measurementCards.get('aux_bat_v')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('aux_bat_v')!.avg(), '00.00') }} {{
                      measurementCards.get('aux_bat_v')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <!-- <v-progress-linear :model-value="measurementCards.get('aux_bat_i')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('aux_bat_i')!.avg(), '00.00') }} {{
                      measurementCards.get('aux_bat_i')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear>

              <v-progress-linear :model-value="measurementCards.get('aux_bat_p')!.avg(true)" height="11px"
                class="pa-0 mb-2 my-1">
                <template v-slot:default="">
                  <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
                    {{ format(measurementCards.get('aux_bat_p')!.avg(), '000.0') }} {{
                      measurementCards.get('aux_bat_p')!.units
                    }}
                  </p>
                </template>
              </v-progress-linear> -->

            </v-col>

          </v-row>

        </v-card>

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

      </v-col>

    </v-row>

    <v-divider class="my-4"></v-divider>

    <v-card>
      <v-card-title>Can Messages</v-card-title>
      <v-card-item v-for="( message, index ) in  messages.values() " :key="index"> {{ message }}</v-card-item>
    </v-card>

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
  offset: 1px;
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
import { Event, listen } from '@tauri-apps/api/event'
import { format } from 'numerable'
import { Ref, reactive, ref } from 'vue'
import { GenericCardData } from '../measurement_types'

// Getting data from the backend
type CanMessage = {
  timestamp: number,
  message: string
}
const messages: Ref<CanMessage[]> = ref([])
await listen('can_message', (event: Event<object>) => {
  let message = event.payload

  // Put message into the debugger
  messages.value.unshift({ timestamp: Date.now(), message: JSON.parse(JSON.stringify(message)) } as CanMessage)
  if (messages.value.length > 10) {
    messages.value.pop()
  }

  // Get all boat data and update each card accordingly
  Object.entries(message).forEach((entry) => {
    const [key, data] = entry

    if (data !== null) {
      const card_instance = measurementCards.get(String(key))
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
})



const measurementCards = reactive(new Map<string, GenericCardData>)
measurementCards.set("motor_d", new GenericCardData(
  "Motor D",
  "ESC PWM Duty-Cycle",
  '%',
  0,
  100,
  1,
  0,
))
measurementCards.set("motor_rpm", new GenericCardData(
  "Motor RPM",
  "Motor RPM",
  'RPM',
  0,
  6000,
  0,
  0,
))
measurementCards.set("bat_v", new GenericCardData(
  "Bat Cell V",
  "Battery Voltage",
  'V',
  30,
  60,
  2,
  0,
))
measurementCards.set("bat_i", new GenericCardData(
  "Bat I",
  "Battery Current",
  'A',
  -200,
  200,
  1,
  0,
))
measurementCards.set("bat_p", new GenericCardData(
  "Bat Cell V",
  "Battery Power",
  'W',
  -10000,
  10000,
  0,
  0,
))
measurementCards.set("bat_cell_v", new GenericCardData(
  "Bat V",
  "Battery Voltage",
  'V',
  10,
  16,
  2,
  'Sum',
))
measurementCards.set("dir_pos", new GenericCardData(
  "Dir H",
  "Steering System Sensors Position",
  '°',
  -135,
  135,
  1,
  0,
))
measurementCards.set("dir_bat_v", new GenericCardData(
  "Dir V",
  "Steering System Battery Voltage",
  'V',
  7,
  15,
  1,
  0,
))
measurementCards.set("dir_bat_i", new GenericCardData(
  "Dir I",
  "Steering System Battery Current",
  'A',
  0,
  20,
  1,
  0,
))
measurementCards.set("dir_bat_p", new GenericCardData(
  "Dir I",
  "Steering System Battery Power",
  'W',
  0,
  300,
  0,
  0,
))
measurementCards.set("mcc_pi", new GenericCardData(
  "MPPTs Pi",
  "MPPTs Input Power",
  'W',
  0,
  300,
  0,
  'Sum',
))
measurementCards.set("mcc_vi", new GenericCardData(
  "MPPT Vi",
  "MPPT Input Voltage",
  'V',
  0,
  60,
  2,
  'NonZeroAverage',
))
measurementCards.set("mcc_ii", new GenericCardData(
  "MPPT Ii",
  "MPPT Input Current",
  'A',
  0,
  15,
  2,
  'NonZeroAverage',
))
measurementCards.set("mcc_vo", new GenericCardData(
  "MPPT Vo",
  "MPPT Output Voltage",
  'V',
  0,
  60,
  1,
  'NonZeroAverage',
))
measurementCards.set("mcc_d", new GenericCardData(
  "MPPT D",
  "MPPT Duty Cycle",
  '%',
  0,
  100,
  1,
  'NonZeroAverage',
))
measurementCards.set("aux_bat_v", new GenericCardData(
  "AUX V",
  "Auxiliar Battery Voltage",
  'V',
  7,
  15,
  1,
  0,
))
measurementCards.set("aux_bat_i", new GenericCardData(
  "AUX V",
  "Auxiliar Battery Voltage",
  'A',
  0,
  20,
  1,
  0,
))
measurementCards.set("aux_bat_p", new GenericCardData(
  "AUX V",
  "Auxiliar Battery Voltage",
  'W',
  0,
  200,
  1,
  0,
))
measurementCards.set("pump", new GenericCardData(
  "BILDGE PUMP",
  "Bildge Pump Status",
  '',
  0,
  1,
  0,
  0,
))
measurementCards.set("boat_on", new GenericCardData(
  "BOAT ON",
  "Boat Status",
  '',
  0,
  1,
  0,
  0,
))
measurementCards.set("dms_on", new GenericCardData(
  "DMS ON",
  "Dead Man Switch Status",
  '',
  0,
  1,
  0,
  0,
))
measurementCards.set("motor_on", new GenericCardData(
  "MOTOR ON",
  "Motor Status",
  '',
  0,
  1,
  0,
  0,
))
measurementCards.set("motor_rev", new GenericCardData(
  "MOTOR REVERSE",
  "Motor Reverse Status",
  '',
  0,
  1,
  0,
  0,
))
measurementCards.set("mam_machine_state", new GenericCardData(
  "MAM MACHINE STATE",
  "MAM Machine State",
  '',
  0,
  1,
  0,
  0,
))

</script>