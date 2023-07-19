<template>
  <v-card class="ma-0 pa-2" variant="outlined" width="280">
    <v-card-subtitle align="right" class="pt-0 pb-2 text-capitalize">{{ card.description }}</v-card-subtitle>
    <v-card-title align="left" class="text-h4 bg-primary font-weight-medium">{{ card.name }}</v-card-title>
    <v-card-text class="py-0">
      <v-row align="end" no-gutters>
        <v-col class="text-h2 text-right text-no-wrap font-weight-black" cols="9">
          {{ format(card) }}
        </v-col>
        <v-col class="text-left text-overline ml-4 text-no-wrap" cols="1">
          [ {{ card.units }} ]
        </v-col>
      </v-row>
    </v-card-text>
    <v-progress-linear :model-value="percentage(card)" color="white" height="25"></v-progress-linear>
  </v-card>
</template>

<script setup lang="ts">

import { MeasurementCardData } from '../measurement_types'

defineProps({
  card: {
    type: MeasurementCardData,
    required: true,
  },
})

const format = (card: MeasurementCardData): string => {
  let str = card.value.toFixed(card.precision).substring(0, 5)

  if (str.charAt(str.length - 1) === '.') {
    str = str.substring(0, str.length - 1)
  }

  return str
}

const percentage = (card: MeasurementCardData): number => {
  if (!card) {
    return 0
  }
  return 100 * (card.value - card.min) / (card.max - card.min)
}

</script>
