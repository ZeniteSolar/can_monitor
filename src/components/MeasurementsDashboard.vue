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
import { reactive, ref } from 'vue'
import { MeasurementCardData } from '../measurement_types'
import MeasurementCard from './MeasurementCard.vue'

// Getting data from the backend
const messages = ref([])
await listen('can_message', (event: Event<object>) => {
  let message = event.payload

  console.log(typeof (message), message)
  messages.value.unshift({ timestamp: Date.now(), message: message })

  if (messages.value.length > 10) {
    messages.value.pop()
  }

  for (const [key, value] of Object.entries(message)) {
    if (value === null) {
      continue
    }

    const card_instance = measurementCards.get(String(key))
    console.log(card_instance)
    if (!card_instance) {
      return
    }
    card_instance.value = value
  }
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
measurementCards.set("bat_ii", new MeasurementCardData(
  "Bat Ii",
  "Battery Input Current",
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
measurementCards.set("mppts_pi", new MeasurementCardData(
  "MPPTs Pi",
  "MPPTs Input Power",
  'W',
  0,
  1500,
  0
));

</script>