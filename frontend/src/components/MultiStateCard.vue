<template>
  <v-card class="ma-0 pa-0 state-card">
    <!-- Title -->
    <v-card-title :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''">
      {{ title }}
    </v-card-title>

    <!-- Three columns: labels, transitions, descriptions -->
    <v-row class="px-4 py-2 state-grid" no-gutters>
      <!-- LABEL COLUMN -->
      <v-col cols="auto">
        <div class="column">
          <div v-for="state in moduleStates" :key="state.label" class="cell label-cell">
            {{ state.label }}
          </div>
        </div>
      </v-col>

      <!-- TRANSITION COLUMN -->
      <v-col cols="auto">
        <div class="column">
          <div v-for="state in moduleStates" :key="state.label" class="cell value-cell">
            {{ getStateLabel(prevStates[state.label], state.label) }} → {{ getStateLabel(state.value, state.label) }}
          </div>
        </div>
      </v-col>

      <!-- DESCRIPTION COLUMN -->
      <!-- <v-col>
        <div class="column">
          <div v-for="state in moduleStates" :key="state.label" class="cell description-cell">
            {{ state.description }}
          </div>
        </div>
      </v-col> -->
    </v-row>
  </v-card>
</template>

<script setup lang="ts">
import type { BoardState } from '@/types/index';
import { defineProps, computed, ref, watch } from 'vue';
import { measurementCards } from '@/measurement_cards';

const props = defineProps<{
  title: string;
  titleColor?: string;
  modules: Array<{ label: string; stateKey: string; errorKey?: string; index?: number }>;
}>();

const prevStates = ref<Record<string, number>>({});

// Descriptions centralized
const moduleDescriptions: Record<string, string[]> = {
  MIC: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCS: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MAM: ['Init', 'Contactor...', 'Idle', 'Running', 'Code <X>'],
  MAC: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MSC_1: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCB_1: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCB_2: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MDE: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
};

function getStateLabel(val: number, label: string) {
  const defaultStates = ['INIT', 'IDLE', 'RUN', 'ERROR', 'RESET'];
  const mamStates = ['INIT', 'CONTAT', 'IDLE', 'RUN', 'ERROR'];
  const isMam = label === 'MAM';
  const list = isMam ? mamStates : defaultStates;
  return list[val] ?? 'UNKNOWN';
}

function getErrorDescription(
  label: string,
  state: number,
  errorCode: number | undefined):
  string {
  const base = moduleDescriptions[label]?.[state];
  return base?.replace('<X>', `${errorCode ?? '?'}`) ?? 'UNKNOWN';
}

const moduleStates = computed<BoardState[]>(() =>
  props.modules.map(({ label, stateKey, errorKey, index = 0 }) => {
    const raw = measurementCards[stateKey]?.data?.[index];
    const value = typeof raw === 'number' ? raw : typeof raw === 'boolean' ? (raw ? 1 : 0) : 0;

    const errorData = measurementCards[errorKey || '']?.data;
    let error: number | undefined = undefined;

    if (Array.isArray(errorData)) {
      const maybe = errorData[index];
      error = typeof maybe === 'number' ? maybe : undefined;
    } else if (typeof errorData === 'number') {
      error = errorData;
    }

    const description = getErrorDescription(label, value, error);
    return { label, value, description };
  })
);

watch(
  moduleStates,
  (newList, oldList) => {
    newList.forEach(({ label, value }) => {
      const old = oldList?.find((s) => s.label === label)?.value;

      if (old !== undefined && old !== value) {
        prevStates.value[label] = old;
      }

      // Fallback for first-time init
      if (prevStates.value[label] === undefined) {
        prevStates.value[label] = value;
      }
    });
  },
  { immediate: true, deep: true }
);

</script>

<style scoped>
.state-card {
  --state-gap: 0.5rem;
  --state-padding: 0.25rem 0.5rem;
  --value-width: 14ch;
}

.state-grid {
  display: grid;
  grid-template-columns: max-content var(--value-width) minmax(0, 1fr);
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
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
