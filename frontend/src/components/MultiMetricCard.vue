<template>
  <v-card class="ma-0 pa-0" :style="{ width: cardWidth }"> <!-- ← bind width here -->
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
          <div class="metric-value-text" :style="{ fontSize: fontSize }">
            {{ format(value, '00.0') }}
            {{ metric.units[idx] || metric.units[0] }}
          </div>
        </div>
      </v-col>
    </component>
    </v-card>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';
import { format } from 'numerable';
import type { BoardMetric, Orientation } from '@/types/index';

const props = defineProps<{
  title: String;
  titleColor?: string;
  metric_fontSize?: string;      // <-- new prop
  cardWidth?: string;          // <-- new prop
  orientation?: Orientation;
  metricsData: (BoardMetric & { plain?: boolean })[];
}>();

const metricsData = computed(() => props.metricsData);

// compute the font-size to use (rem string), default to 1.5rem
const fontSize = computed(() => props.metric_fontSize ?? '1.5rem');
// fixed CSS width for the card, default to 250px
const cardWidth = computed(() => props.cardWidth ?? '300px');
</script>

<style scoped>
.v-card-title {
  font-family: var(--zenite-ui-font);
  text-align: center;
  font-size: 1.5rem;
}

/* labels stay the same */
.label-cell {
  font-weight: thin;
  font-family: var(--zenite-data-font) !important;
  font-size: 1.35rem;
  text-align: center;
  padding: 0;
  margin: 0;
}

.metric-plain-display {
  background-color: #eee;
  border-radius: 13px;
  padding: 7px 4px;
  margin: 2px 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

.metric-value-text {
  font-family: var(--zenite-data-font);
  font-weight: bold;
  text-align: center;
  line-height: 1;
  /* prevents excess height from font */
  /* remove the hardcoded font-size here; use inline style instead */
}
</style>
