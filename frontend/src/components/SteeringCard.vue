<template>
  <v-card class="ma-0 pa-2 steering-card" style="background: lightblue; background-image: repeating-linear-gradient(120deg, rgba(255,255,255,0.4) 0px, rgba(255,255,255,0.4) 2px, transparent 2px, transparent 10px); background-size: 15px 15px; background-repeat: repeat;">
    <v-card-title
      :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''"
    >
      {{ title }}
    </v-card-title>

    <!-- SVG container: simplified stern box + circle + dual pointer arrows -->
    <v-card-text class="d-flex justify-center pa-0">
      <svg width="300" height="150" viewBox="0 0 200 150" xmlns="http://www.w3.org/2000/svg">
        <!-- 0. Rectangle container: orange fill with black outline -->
        <rect
          x="-45"
          y="0"
          width="295"
          height="120"
          fill="orange"
          stroke="black"
          stroke-width="2"
        />
        <!-- 1. Stern section semicircle: radius 100, pivot at (100,0) -->
        <circle
          cx="100"
          cy="0"
          r="100"
          fill="lightblue"
          stroke="black"
          stroke-width="2"
          vector-effect="non-scaling-stroke"
        />

        <!-- 2a. Helm pointer: grey triangle, length 100px -->
        <polygon
          points="95,0 105,0 100,100"
          fill="red"
          stroke="red"
          stroke-width="3"
          stroke-linejoin="round"
          :transform="`rotate(${props.angle} 100 0)`"
          vector-effect="non-scaling-stroke"
        />

        <!-- 2b. Tail pointer: black outline triangle, length 100px -->
        <polygon
          points="95,0 105,0 100,100"
          fill="black"
          stroke="black"
          stroke-width="5"
          stroke-linejoin="round"
          :transform="`rotate(${props.tailAngle} 100 0)`"
          vector-effect="non-scaling-stroke"
        />
      </svg>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Dual-angle props, used directly for SVG transforms
import { defineProps } from 'vue';
const props = defineProps<{ 
  angle: number; 
  tailAngle: number; 
  title: String;
  titleColor?: string;
}>();
</script>

<style scoped>
.steering-card {
  /* override card default background */
  background-color: lightblue !important;
  background-image: repeating-linear-gradient(
    120deg,
    rgba(255,255,255,0.4) 0px,
    rgba(255,255,255,0.4) 2px,
    transparent 2px,
    transparent 10px
  ) !important;
  background-repeat: repeat !important;
  background-size: 15px 15px !important;
}
</style>
