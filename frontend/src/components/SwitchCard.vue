<template>
  <v-card class="ma-0 pa-0">
    <v-card-title :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''">
      {{ title }}
    </v-card-title>
    <v-row class="px-2 ma-0">
  <v-col cols="6" class="ma-0 pa-1">
    <v-switch
      v-for="item in props.data.slice(0, Math.ceil(props.data.length / 2))"
      :key="item.label"
      :true-value="true"
      :model-value="item.value"
      :label="item.label"
      color="red"
      readonly
      inset
      hide-details
    />
  </v-col>
  <v-col cols="6" class="ma-0 pa-1">
    <v-switch
      v-for="item in props.data.slice(Math.ceil(props.data.length / 2))"
      :key="item.label"
      :true-value="true"
      :model-value="item.value"
      :label="item.label"
      color="red"
      readonly
      inset
      hide-details
    />
  </v-col>
</v-row>

  </v-card>
</template>


<script setup lang="ts">
import { defineProps, computed } from 'vue';
import type { BoardBoolean } from '@/types/index';

const props = defineProps<{
  title: string,
  titleColor?: string,
  data: BoardBoolean[],
  maxLines: number
}>();

const chunks = computed(() => {
  const chunked = [];
  for (let i = 0; i < props.data.length; i += props.maxLines) {
    chunked.push(props.data.slice(i, i + props.maxLines));
  }
  return chunked;
});

</script>

<style scoped>
.v-card-title {
  font-family: var(--zenite-ui-font);
  text-align: center;
}

</style>
