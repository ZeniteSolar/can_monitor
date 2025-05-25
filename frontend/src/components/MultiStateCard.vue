<template>
  <v-card class="ma-0 pa-0 state-card">
    <!-- Title -->
    <v-card-title
      :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''"
    >
      {{ title }}
    </v-card-title>

    <!-- Three columns: labels, transitions, descriptions -->
    <v-row class="px-4 py-2 state-grid" no-gutters>
      <!-- LABEL COLUMN -->
      <v-col cols="auto">
        <div class="column">
          <div
            v-for="state in stateData"
            :key="state.label"
            class="cell label-cell"
          >
            {{ state.label }}
          </div>
        </div>
      </v-col>

      <!-- TRANSITION COLUMN -->
      <v-col cols="auto">
        <div class="column">
          <div
            v-for="state in stateData"
            :key="state.label"
            class="cell value-cell"
          >
            {{ getStateLabel(prevStates[state.label], state.label) }} → {{ getStateLabel(state.value, state.label) }}
          </div>
        </div>
      </v-col>

      <!-- DESCRIPTION COLUMN -->
      <v-col>
        <div class="column">
          <div
            v-for="state in stateData"
            :key="state.label"
            class="cell description-cell"
          >
            {{ getDescription(state.value, state.label) }}
          </div>
        </div>
      </v-col>
    </v-row>
  </v-card>
</template>

<script setup lang="ts">
import type { BoardState } from '@/types/index';
import { defineProps, ref, watch } from 'vue';

// Props
const props = defineProps<{
  title: string;
  titleColor?: string;
  stateData: Array<BoardState & { description?: string }>;
}>();

// Track previous state values in a reactive map
const prevStates = ref<Record<string, number>>({});
  watch(
  () => props.stateData,
  (newList, oldList = [] as typeof props.stateData) => {
    newList.forEach(({ label, value }) => {
      // On first run (oldList empty), seed prevStates to the current value
      if (oldList.length === 0) {
        prevStates.value[label] = value;
      } else {
        // Find the previous value for this label
        const oldItem = oldList.find(item => item.label === label);
        const oldValue = oldItem?.value ?? prevStates.value[label];

        // Only update prevStates if the state really changed
        if (value !== oldValue) {
          prevStates.value[label] = oldValue!;
        }
      }
    });
  },
  { immediate: true, deep: true }
);

// State-label mappings per module or default
interface LabelMap { [key: string]: string[]; }
const stateLabelsMap: LabelMap = {
  default: ['INIT', 'IDLE', 'RUN', 'ERROR', 'RESET'],
  MAM:     ['INIT', 'CONTAT', 'IDLE', 'RUN', 'ERROR'],
};

// Module-specific description arrays
const moduleDescriptions: LabelMap = {
  MIC: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MCS: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MAM: ['Init', 'Wait Contactor', 'Idle: checking system...', 'Running!', 'Error code: XXX'],
  MAC: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MSC_1: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MCB_1: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MCB_2: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
  MDE: ['Init', 'Idle: checking system...', 'Running!', 'Error code: XXX', 'Reseting'],
};

// Helper: get display name for any state index
function getStateLabel(val: number, label: string) {
  const list = stateLabelsMap[label] ?? stateLabelsMap.default;
  return list[val] ?? 'UNKNOWN';
}

// Helper: description, with per-item override
function getDescription(val: number, label: string) {
  const override = props.stateData.find(s => s.label === label)?.description;
  return override ?? moduleDescriptions[label]?.[val] ?? 'NONE';
}
</script>

<style scoped>
.state-card {
  --state-gap:     0.5rem;
  --state-padding: 0.25rem 0.5rem;
  --value-width:   14ch; /* adjust to fit your longest transition text */
}

/* Grid with fixed second column width */
.state-grid {
  display: grid;
  grid-template-columns: max-content var(--value-width) 1fr;
  column-gap: var(--state-gap);
}

.column {
  display: flex;
  flex-direction: column;
  gap: var(--state-gap);
}

.cell {
  padding: var(--state-padding);
  text-align: left;
}

.label-cell {
  font-weight: bold;
  font-family: var(--zenite-ui-font) !important;
}

.value-cell {
  font-family: var(--zenite-data-font) !important;
  white-space: nowrap;
  box-sizing: border-box;
}

.description-cell {
  font-style: italic;
  font-family: var(--zenite-ui-font) !important;
}
</style>
