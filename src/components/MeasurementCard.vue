<template>
  <v-card class="ma-1 pa-0 pb-2" variant="outlined" width="200">
    <v-card-title align="left" class="text-h5 bg-primary font-weight-medium py-0">{{ card.name }}</v-card-title>
    <v-card-text class="py-0">
      <v-row align="end" no-gutters>
        <v-col class="text-h3 text-right text-no-wrap font-weight-black" cols="8">
          {{ format(card) }}
        </v-col>
        <v-col class="text-left text-overline ml-2 text-no-wrap" cols="2">
          [ {{ card.units }} ]
        </v-col>
      </v-row>
    </v-card-text>
    <v-progress-linear v-for="(_, index) in card.data.values()" :key="index" :model-value="percentage(card, index)"
      color="white" :height="Math.ceil(25 / card.data.length)">
    </v-progress-linear>

  </v-card>
</template>

<script setup lang="ts">
import { MeasurementCardData } from "../measurement_types"

defineProps({
  card: {
    type: MeasurementCardData,
    required: true,
  },
})

const fixate = (value: number, precision: number): string => {
  let str = value.toFixed(precision).substring(0, 5)
  if (str.charAt(str.length - 1) === ".") {
    str = str.substring(0, str.length - 1)
  }

  return str
}

// TODO: handle different data types
const format = (card: MeasurementCardData): string => {
  switch (card.rule) {
    case 'NonZeroAverage': {
      let avg = computeAverageWithoutZeros(card.data)
      return fixate(avg, card.precision)
    }
    case 'Sum': {
      let sum = card.data.reduce((acc, num) => acc + num, 0)
      return fixate(sum, card.precision)
    }
    default: {
      if (typeof (card.rule) !== "number") {
        return "N/A"
      }

      let index = card.rule
      return fixate(card.data[index], card.precision)
    }
  }
}

function computeAverageWithoutZeros(numbers: number[]): number {
  const nonZeroNumbers: number[] = numbers.filter((num) => num !== 0)

  if (nonZeroNumbers.length === 0) {
    return 0 // Avoid division by zero for an array with all zeros or an empty array
  }

  const sum: number = nonZeroNumbers.reduce((acc, num) => acc + num, 0)
  const average: number = sum / nonZeroNumbers.length
  return average
}

const percentage = (card: MeasurementCardData, index: number): number => {
  if (!card) {
    return 0
  }
  return (100 * (card.data[index] - card.min)) / (card.max - card.min)
}
</script>
