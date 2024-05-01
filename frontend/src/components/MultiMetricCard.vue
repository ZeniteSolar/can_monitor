<template>
  <v-card class="ma-0 pa-0">
    <v-card-title class="py-0 mt-0 bg-primary font-weight-black">{{ title }}</v-card-title>
    <v-row class="px-2 ma-0">
    <v-col class="ma-0 pa-1" v-for="(metric, index) in metrics" :key="index">
      
      <v-card-text align="center" class="font-weight-bold text-caption pa-0 ma-0">{{ metric.label }}</v-card-text>

      <div v-for="(value, idx) in metric.data" :key="`${metric.label}-${idx}`">
      <MetricDisplay
        :value="value"
        :units="metric.units"
      />
      </div>
    </v-col>
    </v-row>
  </v-card>
</template>


<script setup>
  import MetricDisplay from './MetricDisplay.vue';
  import { defineProps, computed } from 'vue';
  
  const props = defineProps({
    title: String,
    metricsData: Object
  });
  
  const metrics = computed(() => {
  return Object.entries(props.metricsData).map(([key, values]) => ({
    label: key.charAt(0).toUpperCase() + key.slice(1),
    data: values ?? [],
    units: values.units // Assuming 'units' is passed within each metrics data as an attribute
  }));
  });
  </script>
  
  <style scoped>
</style>
