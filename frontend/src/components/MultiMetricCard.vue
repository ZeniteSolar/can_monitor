<template>
  <v-card class="ma-0 pa-0">
    <v-card-title :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''">
      {{ title }}
    </v-card-title>

    <component :is="orientation === 'VERTICAL' ? 'v-col' : 'v-row'" class="px-2 ma-0 pa-1">
      <v-col class="ma-0 pa-1" v-for="(metric, index) in metricsData" :key="index">
        <v-card-text v-if="metric.label" align="center" class="label-cell">
          {{ metric.label }}
        </v-card-text>

        <div v-for="(value, idx) in metric.data" :key="`${metric.label}-${idx}`" class="metric-plain-display">
          <div class="metric-value-text">
            {{ format(value, '00.0') }} {{ metric.units[idx] || metric.units[0] }}
          </div>
        </div>

      </v-col>
    </component>
  </v-card>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';
// import MetricDisplay from './MetricDisplay.vue';
import { format } from 'numerable';
import type { BoardMetric, Orientation } from '@/types/index';

const props = defineProps<{
  title: String,
  titleColor?: string,
  orientation?: Orientation,
  metricsData: (BoardMetric & { plain?: boolean })[],
}>();

const metricsData = computed(() => props.metricsData);
</script>

<style scoped>
.label-cell {
  font-weight: bold;
  font-family: var(--zenite-ui-font) !important;
  font-size: 0.75rem;
  text-align: center;
  padding: 0;
  margin: 0;
}

.metric-plain-display {
  background-color: #eee;
  border-radius: 6px;
  padding: 1px 4px;
  margin: 2px 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

.metric-value-text {
  font-family: var(--zenite-data-font);
  font-size: 0.75rem;
  font-weight: bold;
  text-align: center;
  line-height: 1;           /* prevents excess height from font */
}

</style>
