<template>
  <v-col class="ma-0 pa-1">
    <v-card-text class="font-weight-bold text-caption pa-0 ma-0"> {{ label }} </v-card-text>
    <v-radio-group :model-value="formattedValue"
      readonly class="data-font mx-5"
      style="font-size: 15px;">
      <v-radio color="orange" :value="0">INIT</v-radio>
      <v-radio color="orange" :value="1">IDLE</v-radio>
      <v-radio color="green" :value="2">RUN</v-radio>
      <v-radio color="red" :value="3">ERROR</v-radio>
      <v-radio color="red" :value="4">RESET</v-radio>
    </v-radio-group>
  </v-col>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  label: String,
  value: Number,
  showLabel: {
    type: Boolean,
    default: true
  }
});

const formattedValue = computed(() => props.value);
</script>

<style scoped>
  .invert {
  mix-blend-mode: difference;
  .data-font {
    font-family: var(--zenite-data-font) !important;
  }
}
</style>