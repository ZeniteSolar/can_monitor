<template>
  <v-container>
    <v-row no-gutters>
      <v-col v-for="(card, index) in measurementCards.values()" :key="index" md="3" align="center">
        <measurement-card :card="card"></measurement-card>
      </v-col>
    </v-row>

    <v-divider class="my-4"></v-divider>

    <v-card>
      <v-card-title>Can Messages</v-card-title>
      <v-card-item v-for="(message, index) in messages.values()" :key="index"> {{ message }}</v-card-item>
    </v-card>

  </v-container>
</template>

<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event'
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
measurementCards.set("bat_i", new GenericCardData(
  "Bat I",
  "Battery Current",
  'A',
  -200,
  200,
  1,
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
  "Steering Wheel Sensors Position",
  'deg',
  -135,
  135,
  1,
  0,
))
measurementCards.set("dir_bat_v", new GenericCardData(
  "Dir V",
  "Steering Wheel Battery Voltage",
  'V',
  7,
  15,
  1,
  0,
))
measurementCards.set("dir_bat_i", new GenericCardData(
  "Dir I",
  "Steering Wheel Battery Current",
  'A',
  0,
  20,
  1,
  0,
))
measurementCards.set("mppt_pi", new GenericCardData(
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
));

</script>