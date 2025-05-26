<template>
  <v-card class="ma-0 pa-0">
    <v-card-title :class="['py-0 mt-0 font-weight-black', titleColor ?? 'bg-primary text-black']"
      :style="titleColor?.includes('text-black') ? 'color: black !important;' : ''">
      {{ title }}
    </v-card-title>

    <!-- Legend -->
    <v-card-text class="d-flex justify-center align-center pa-1" style="gap: 1rem;">
      <div class="legend-box" style="background: red;"></div><span class="legend-text">MIC</span>
      <div class="legend-box" style="background: black;"></div><span class="legend-text">MAM</span>
    </v-card-text>

    <v-card-text class="d-flex justify-center pt-0 pb-2">
      <svg :width="svgWidth" :height="svgHeight" :viewBox="`0 0 ${svgWidth} ${svgHeight}`" xmlns="http://www.w3.org/2000/svg">
        <!-- Light grey background fill -->
        <path :d="filledArcPath" fill="#f0f0f0" stroke="none" />

        <!-- Arc stroke -->
        <path :d="arcPath" fill="none" stroke="#ccc" stroke-width="2" />

        <!-- Horizontal base line -->
        <line
          :x1="polarToCartesian(cx, cy, radius, 0).x"
          :y1="cy"
          :x2="polarToCartesian(cx, cy, radius, 180).x"
          :y2="cy"
          stroke="#aaa"
          stroke-width="1"
        />

        <!-- Tick marks and labels -->
        <g v-for="step in ticks" :key="step">
          <line
            :x1="tickX(step,10)"
            :y1="tickY(step,10)"
            :x2=cx
            :y2=cy
            stroke="#888"
            stroke-width="1"
            stroke-dasharray="10,6"
          />
          <text
            :x="labelX(step)"
            :y="labelY(step)"
            text-anchor="middle"
            alignment-baseline="middle"
            fill="black"
            font-size="12"
          >
            {{ step }}%
          </text>
        </g>

        <!-- MIC Pointer -->
        <polygon
          :points="pointerPoints(steeringAngle, radius * 0.75)"
          fill="red"
          stroke="red"
          stroke-width="1"
        />

        <!-- MAM Pointer -->
        <polygon
          :points="pointerPoints(tailAngle, radius)"
          fill="black"
          stroke="black"
          stroke-width="2"
        />

        <!-- Base circle -->
        <circle :cx="cx" :cy="cy" r="6" fill="black" />

      </svg>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';

const props = defineProps<{
  title: string;
  titleColor?: string;
  data: number[]; // [mic, mam]
}>();

const svgWidth = 250;
const svgHeight = 150;
const margin = 20;
const cx = svgWidth / 2;
const cy = svgHeight - margin;
const radius = 90;
const ticks = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

const steeringAngle = computed(() =>
  typeof props.data[0] === 'number' ? (props.data[0] / 100) * 180 : 0
);

const tailAngle = computed(() =>
  typeof props.data[1] === 'number' ? (props.data[1] / 100) * 180 : 0
);

const arcPath = computed(() => {
  const start = polarToCartesian(cx, cy, radius, 0);
  const end = polarToCartesian(cx, cy, radius, 180);
  return `M ${start.x} ${start.y} A ${radius} ${radius} 0 1 1 ${end.x} ${end.y}`;
});

const filledArcPath = computed(() => {
  const start = polarToCartesian(cx, cy, radius, 0);
  const end = polarToCartesian(cx, cy, radius, 180);
  return `
    M ${start.x} ${start.y}
    A ${radius} ${radius} 0 1 1 ${end.x} ${end.y}
    L ${cx} ${cy}
    Z
  `;
});

function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
  const angleRad = (Math.PI * angleDeg) / 180;
  return {
    x: cx + r * Math.cos(angleRad - Math.PI),
    y: cy + r * Math.sin(angleRad - Math.PI),
  };
}

function pointerPoints(angle: number, len: number): string {
  const angleRad = (Math.PI * angle) / 180 - Math.PI;
  const baseLeft = polarToCartesian(cx, cy, 5, angle - 90);
  const baseRight = polarToCartesian(cx, cy, 5, angle + 90);
  const tip = {
    x: cx + len * Math.cos(angleRad),
    y: cy + len * Math.sin(angleRad),
  };
  return `${baseLeft.x},${baseLeft.y} ${baseRight.x},${baseRight.y} ${tip.x},${tip.y}`;
}

function tickX(step: number, offset = 0) {
  return polarToCartesian(cx, cy, radius + offset, (step / 100) * 180).x;
}
function tickY(step: number, offset = 0) {
  return polarToCartesian(cx, cy, radius + offset, (step / 100) * 180).y;
}
function labelX(step: number) {
  return polarToCartesian(cx, cy, radius + 18, (step / 100) * 180).x;
}
function labelY(step: number) {
  return polarToCartesian(cx, cy, radius + 18, (step / 100) * 180).y;
}
</script>

<style scoped>
.v-card-title {
  font-family: var(--zenite-ui-font);
  text-align: center;
}

.legend-box {
  width: 15px;
  height: 15px;
  display: inline-block;
  border: 1px solid black;
}
.legend-text {
  font-family: var(--zenite-ui-font);
  font-size: 1rem;
}
</style>
