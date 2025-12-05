<template>
  <v-card class="ma-0 pa-0" :class="cardClass" :style="{ width: cardWidth }">
    <v-card-title class="py-2 font-weight-black text-center" :class="titleColor">
      {{ title }}
    </v-card-title>

    <v-card-text class="pa-3 text-center">
      <div class="status-display">
        <p class="text-h5 font-weight-bold">
          {{ statusText }}
        </p>
        <p v-if="errorCode && errorCode !== 0" class="text-error text-caption mt-1">
          Código de erro: {{ errorCode }}
        </p>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';

const props = defineProps<{
  title: string;
  titleColor?: string;
  zeniraEnabled?: boolean | null;
  listeningState?: number | null;
  errorCode?: number | null;
  cardWidth?: string;
}>();

const statusText = computed(() => {
  // console.log('VoiceControlCard debug:', { zeniraEnabled: props.zeniraEnabled, listeningState: props.listeningState });
  if (!props.zeniraEnabled || props.zeniraEnabled === null || props.zeniraEnabled === undefined) {
    return 'DESLIGADO';
  }
  return props.listeningState === 1 ? 'ESCUTANDO' : 'AGUARDANDO';
});

const cardClass = computed(() => {
  if (!props.zeniraEnabled || props.zeniraEnabled === null || props.zeniraEnabled === undefined) {
    return 'bg-grey-lighten-5';
  }
  if (props.listeningState === 1) {
    return 'bg-blue-lighten-4 listening-active';
  }
  return 'bg-grey-lighten-5';
});

const cardWidth = computed(() => props.cardWidth ?? '100%');
</script>

<style scoped>
.listening-active {
  border: 2px solid #1976d2;
  background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%) !important;
}

.status-display {
  padding: 1rem 0;
}
</style>

