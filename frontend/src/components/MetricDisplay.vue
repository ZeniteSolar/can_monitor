<template>
    <v-card-text v-if="showLabel" align="center" class="font-weight-bold text-caption pa-0 ma-0">{{ label }}</v-card-text>
    <v-progress-linear :model-value="value" height="11px" class="pa-0 my-1">
    <template v-slot:default>
      <p class="text-caption invert ma-0" style="padding-bottom: 2px; padding-top: 1px;">
      {{ formattedValue }} {{ units }}
      </p>
    </template>
    </v-progress-linear>
</template>

<script setup>
import { computed } from 'vue';
import { format } from 'numerable'

const props = defineProps({
  label: String,
  value: [Number, String],
  units: String,
  showLabel: {
    type: Boolean,
    default: true
  }
});

const formattedValue = computed(() => format(props.value, '00.0'));
</script>

<style scoped>
  .invert {
  mix-blend-mode: difference;
}
</style>
