<template>
  <v-card class="ma-0 pa-0">
    <v-card-title class="py-0 mt-0 bg-primary font-weight-black">{{ title }}</v-card-title>
    <v-row class="px-2 ma-0">
    <v-col class="ma-0 pa-1" v-for="(metric, index) in metrics" :key="index">
      
      <v-card-text v-if="metric.label" align="center" class="font-weight-bold text-caption pa-0 ma-0">{{ metric.label }}</v-card-text>

      <div v-for="(value, idx) in metric.data" :key="`${metric.label}-${idx}`">
      <MetricDisplay
        :value="value"
        :units="metric.units[idx] || metric.units[0]"
      />
      </div>
    </v-col>
    </v-row>
  </v-card>
</template>


<script setup lang="ts">
  import { defineProps, computed } from 'vue';
  import MetricDisplay from './MetricDisplay.vue';
  /* Types */
  import type { BoardMetric, Orientation } from '@/types/index';

  
  const props = defineProps<{
    title: String,
    metricsData: BoardMetric[]
  }>();

  const metricsData = computed(() => {
    return props.metricsData;
  });

  </script>
  
  <style scoped>
</style>
