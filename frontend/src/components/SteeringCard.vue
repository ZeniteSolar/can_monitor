<template>
  <v-card class="ma-0 pa-0 steering-card">
    <v-card-title
      :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''"
    >
      {{ title }}
    </v-card-title>

    <!-- Wrap only this in background -->
    <div class="steering-body px-2 pb-2 pt-1">
      <v-card-text class="d-flex justify-center pa-0">
        <svg width="400" height="150" viewBox="0 0 200 150" xmlns="http://www.w3.org/2000/svg">
          <!-- SVG content stays the same -->
          <mask id="rectMask">
            <rect x="-75" y="0" width="345" height="140" fill="white" />
            <circle cx="100" cy="0" r="100" fill="black" />
          </mask>
          <rect x="-75" y="0" width="345" height="140" rx="10" ry="10"
            fill="orange" stroke="black" stroke-width="2" mask="url(#rectMask)" />
          <circle cx="100" cy="0" r="100" fill="none" stroke="black" stroke-width="2"
            vector-effect="non-scaling-stroke" />
          <rect x="50" y="105" width="100" height="30" rx="5" ry="5" fill="white" stroke="black" stroke-width="1" />
          <text x="100" y="125" text-anchor="middle" fill="black"
            font-family="sans-serif" font-size="16">Guarapuvu II</text>
          <polygon points="95,0 105,0 100,100" fill="red" stroke="red" stroke-width="2" stroke-linejoin="round"
            :transform="`rotate(${props.steeringAngle} 100 0)`" vector-effect="non-scaling-stroke" />
          <polygon points="95,0 105,0 100,100" fill="black" stroke="black" stroke-width="5" stroke-linejoin="round"
            :transform="`rotate(${props.tailAngle} 100 0)`" vector-effect="non-scaling-stroke" />
        </svg>
      </v-card-text>
    </div>
  </v-card>
</template>

<script setup lang="ts">
// Props coming from the dashboard
import { defineProps } from 'vue';

const props = defineProps<{
  steeringAngle: number;
  tailAngle: number;
  title: string;
  titleColor?: string;
}>();
</script>

<style scoped>
.steering-body {
  background-color: lightblue;
  background-image: repeating-linear-gradient(
    120deg,
    rgba(255, 255, 255, 0.4) 0px,
    rgba(255, 255, 255, 0.4) 2px,
    transparent 2px,
    transparent 10px
  );
  background-size: 15px 15px;
  background-repeat: repeat;
}

.v-card-title {
  border-radius: 0 !important;
}

</style>
