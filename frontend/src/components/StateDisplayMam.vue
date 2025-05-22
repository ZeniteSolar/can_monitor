<template>
  <v-col class="ma-0 pa-1">
  <v-card-text class="text-mono font-weight-bold text-caption pa-0 ma-0"> MAM </v-card-text>
  <v-radio-group :model-value="formattedValue"
    readonly class="text-mono mx-5" style="font-size:15px">
    <v-radio color="orange" value="0">INIT</v-radio>
    <v-radio color="orange" value="1">CONTAT</v-radio>
    <v-radio color="orange" value="2">IDLE</v-radio>
    <v-radio color="green" value="3">RUN</v-radio>
    <v-radio color="red" value="4">ERROR</v-radio>
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

const formattedValue = computed(() => String(props.value));
</script>

<style scoped>
  .invert {
  mix-blend-mode: difference;
}
</style>
