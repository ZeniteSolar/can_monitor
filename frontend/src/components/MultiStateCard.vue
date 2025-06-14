<template>
 <v-card class="ma-0 pa-0 state-card"
         :style="{ width: cardWidth }">   <!-- ← bind width here -->
    <v-card-title :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''">
      {{ title }}
    </v-card-title>

    <v-row class="px-4 py-2 state-grid" no-gutters>
      <v-col cols="auto">
        <div class="column">
          <div v-for="state in moduleStates" :key="state.label" class="cell label-cell">
            {{ state.label }}
          </div>
        </div>
      </v-col>

      <v-col cols="auto">
        <div class="column">
          <div
            v-for="state in moduleStates"
            :key="state.label"
            class="cell value-cell"
            :class="{ disconnected: state.value === DISC }"
          >
            {{ getStateLabel(prevStates[state.label], state.label) }} →
            {{ getStateLabel(state.value, state.label) }}
          </div>
        </div>
      </v-col>
    </v-row>
  </v-card>
</template>

<script setup lang="ts">
import type { BoardState } from '@/types/index';
import { defineProps, computed, ref, watch, onMounted, onUnmounted } from 'vue';
import { measurementCards } from '@/measurement_cards';

const props = defineProps<{
  title: string;
  titleColor?: string;
  cardWidth?: string;          // <-- new prop
  modules: Array<{ label: string; stateKey: string; errorKey?: string; index?: number }>;
}>();

const prevStates = ref<Record<string, number>>({});
const lastUpdateTimes = ref<Record<string, number>>({});
const DISC = -1;
const cardWidth = computed(() => props.cardWidth ?? '300px');
const currentValues = ref<Record<string, number>>({});

// States map and update tracking
const moduleStates = computed<BoardState[]>(() => {
  return props.modules.map(({ label, stateKey, errorKey, index = 0 }) => {
    const raw = measurementCards[stateKey]?.data?.[index];
    const now = Date.now();

    if (typeof raw === 'number') {
      currentValues.value[label] = raw;
      lastUpdateTimes.value[label] = now;
    }

    const lastSeen = measurementCards[stateKey]?.__touched__;
    const disconnected = !lastSeen || (Date.now() - lastSeen > 100); // in ms
    const value = disconnected ? DISC : currentValues.value[label]!;

    const errorData = measurementCards[errorKey || '']?.data;
    let error: number | undefined = undefined;

    if (Array.isArray(errorData)) {
      const maybe = errorData[index];
      error = typeof maybe === 'number' ? maybe : undefined;
    } else if (typeof errorData === 'number') {
      error = errorData;
    }

    return {
      label,
      value,
      description: getErrorDescription(label, value, error),
    };
  });
});

function getStateLabel(val: number, label: string): string {
  if (val === DISC) return 'DISC';
  const defaultStates = ['INIT', 'IDLE', 'RUN', 'ERROR', 'RESET'];
  const mamStates = ['INIT', 'CONTAT', 'IDLE', 'RUN', 'ERROR'];
  const states = label === 'MAM' ? mamStates : defaultStates;
  return states[val] ?? 'UNKNOWN';
}

function getErrorDescription(label: string, state: number, errorCode?: number): string {
  const base = moduleDescriptions[label]?.[state];
  return base?.replace('<X>', `${errorCode ?? '?'}`) ?? 'UNKNOWN';
}

const moduleDescriptions: Record<string, string[]> = {
  MIC: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCS: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MAM: ['Init', 'Contactor...', 'Idle', 'Running', 'Code <X>'],
  MAC: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MSC_1: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MSC_4: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCB_1: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MCB_2: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
  MDE: ['Init', 'Idle', 'Running', 'Code <X>', 'Reseting'],
};

watch(
  moduleStates,
  (newList, oldList) => {
    newList.forEach(({ label, value }) => {
      const old = oldList?.find((s) => s.label === label)?.value;
      if (old !== undefined && old !== value) {
        prevStates.value[label] = old;
      }
      if (prevStates.value[label] === undefined) {
        prevStates.value[label] = value;
      }
    });
  },
  { immediate: true, deep: true }
);

// Optional: cleanup logic if needed later
onMounted(() => {
  // You can poll or handle time-based disconnects here if needed
});

onUnmounted(() => {
  // Cleanup if needed
});
</script>

<style scoped>
.v-card-title {
  font-family: var(--zenite-ui-font);
  text-align: center;
  font-size: 1rem;
}

.state-card {
  --state-gap: 0.5rem;
  --state-padding: 0.25rem 0.5rem;
  --value-width: 14ch;
}

.state-grid {
  display: grid;
  grid-template-columns: max-content var(--value-width) minmax(0, 1fr);
  column-gap: var(--state-gap);
  font-size: 1rem;
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

.disconnected {
  color: #ff3b3b;
  font-weight: bold;
}

.description-cell {
  font-style: italic;
  font-family: var(--zenite-ui-font) !important;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
