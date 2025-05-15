<template>
  <v-card class="ma-0 pa-0">
    <v-card-title
  :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
  :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''"
>
  {{ title }}
</v-card-title>
    <component :is="orientation === 'VERTICAL' ? 'v-col' : 'v-row'" class="px-2 ma-0 ma-0 pa-1" >
    <v-col class="ma-0 pa-1" v-for="(metric, index) in metricsData" :key="index">
      
      <v-card-text v-if="metric.label" align="center" class="text-mono font-weight-bold text-caption pa-0 ma-0">{{ metric.label }}</v-card-text>

      <div v-for="(value, idx) in metric.data" :key="`${metric.label}-${idx}`">
      <MetricDisplay
        :value="value"
        :units="metric.units[idx] || metric.units[0]"
      />
      </div>
    </v-col>
  </component>
  </v-card>
</template>


<script setup lang="ts">
  import { defineProps, computed } from 'vue';
  import MetricDisplay from './MetricDisplay.vue';
  /* Types */
  import type { BoardMetric, Orientation } from '@/types/index';

  
  const props = defineProps<{
    title: String,
    titleColor?: string
    orientation?: Orientation,
    metricsData: BoardMetric[],
  }>();

  const metricsData = computed(() => {
    return props.metricsData;
  });

  </script>
  
  <style scoped>
</style>
