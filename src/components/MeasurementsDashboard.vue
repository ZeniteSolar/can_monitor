<template>
  <v-container>
    <v-row no-gutters>
      <v-col v-for="(card, index) in measurementCards.values()" :key="index" cols="12" md="4" sm="4" align="center">
        <measurement-card :card="card"></measurement-card>
      </v-col>
    </v-row>

    <v-divider class="my-2"></v-divider>

    <v-card>
      <v-card-title>Can Messages</v-card-title>
      <v-card-item v-for="(message, index) in messages.values()" :key="index"> {{ message }}</v-card-item>
    </v-card>

  </v-container>
</template>

<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event'
import { Ref, reactive, ref } from 'vue'
import { MeasurementCardData } from '../measurement_types'
import MeasurementCard from './MeasurementCard.vue'

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
    const [key, value] = entry

    if (value !== null) {
      const card_instance = measurementCards.get(String(key))
      if (card_instance) {
        card_instance.value = value
      }
    }

  })
})

// Mocked backend for development
// const mockTimer = setInterval(() => {
//   const measurements = ["bat_ii"]
//   measurements.forEach((measurement: string): void => {
//     const card_instance = measurementCards.get(measurement)
//     if (!card_instance) {
//       return
//     }
//     card_instance.value = (Math.random() * (card_instance.max - card_instance.min)) + card_instance.min
//   })
// }, 1000)
// onBeforeUnmount(async () => {
//   clearInterval(mockTimer)
// })

const measurementCards = reactive(new Map<string, MeasurementCardData>)
measurementCards.set("motor_d", new MeasurementCardData(
  "Motor D",
  "ESC PWM Duty-Cycle",
  '%',
  0,
  100,
  1
))
measurementCards.set("motor_rpm", new MeasurementCardData(
  "Motor RPM",
  "Motor RPM",
  'RPM',
  0,
  6000,
  0
))
measurementCards.set("bat_ii", new MeasurementCardData(
  "Bat Ii",
  "Battery Input Current",
  'A',
  0,
  60,
  1
))
measurementCards.set("bat_io", new MeasurementCardData(
  "Bat Io",
  "Battery Output Current",
  'A',
  0,
  200,
  1
))
measurementCards.set("bat_v", new MeasurementCardData(
  "Bat V",
  "Battery Voltage",
  'V',
  30,
  60,
  2
))
measurementCards.set("dir_head_pos", new MeasurementCardData(
  "Dir H",
  "Steering Wheel Head Sensor Position",
  'deg',
  -135,
  135,
  1
))
measurementCards.set("dir_tail_pos", new MeasurementCardData(
  "Dir T",
  "Steering Wheel Tail Sensor Position",
  'deg',
  -135,
  135,
  1
))
measurementCards.set("dir_bat_v", new MeasurementCardData(
  "Dir V",
  "Steering Wheel Battery Voltage",
  'V',
  7,
  15,
  1
))
measurementCards.set("dir_bat_i", new MeasurementCardData(
  "Dir I",
  "Steering Wheel Battery Current",
  'A',
  0,
  20,
  1
))
measurementCards.set("mppts_pi", new MeasurementCardData(
  "MPPTs Pi",
  "MPPTs Input Power",
  'W',
  0,
  1500,
  0
));

</script>