<template>
  <v-card class="ma-0 pa-0">
    <v-card-title class="py-0 mt-0 bg-primary font-weight-black">{{ title }}</v-card-title>
    <v-row class="px-2 ma-0">
      <template v-for="(state, index) in stateData" :key="index">
        <StateDisplayMam
          v-if="state.label === 'MAM'"
          :label="state.label"
          :value="state.value"
        />
        <StateDisplay
          v-else
          :label="state.label"
          :value="state.value"
        />
      </template>
    </v-row>
  </v-card>
</template>

<script setup lang="ts">
  import StateDisplay from './StateDisplay.vue';
  import StateDisplayMam from './StateDisplayMam.vue';
  import type { BoardState } from '@/types/index';
  import { computed } from 'vue';



  const props = defineProps<{
    title: string;
    stateData: BoardState[];
  }>();

</script>

<style scoped>
</style>
