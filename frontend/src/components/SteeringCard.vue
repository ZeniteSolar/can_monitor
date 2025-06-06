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
        <svg
          :width="svgWidth"
          :height="svgHeight"
          :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
          xmlns="http://www.w3.org/2000/svg"
        >
          <!-- Mask and background arc -->
          <mask id="rectMask">
            <rect
              :x="cx - radius - 70"
              y="0"
              :width="radius * 2 + 140"
              height="140"
              fill="white"
            />
            <circle :cx="cx" cy="0" :r="radius" fill="black" />
          </mask>

          <rect
            :x="cx - radius - 70"
            y="0"
            :width="radius * 2 + 140"
            height="140"
            rx="10"
            ry="10"
            fill="orange"
            stroke="black"
            stroke-width="2"
            mask="url(#rectMask)"
          />

          <circle :cx="cx" cy="0" :r="radius" fill="none" stroke="black" stroke-width="2" />

          <!-- Boat label -->
          <rect
            :x="cx - 50"
            y="105"
            width="100"
            height="30"
            rx="5"
            ry="5"
            fill="white"
            stroke="black"
            stroke-width="1"
          />
          <text
            :x="cx"
            y="125"
            text-anchor="middle"
            fill="black"
            font-family="sans-serif"
            font-size="16"
          >
            Guarapuvu II
          </text>

          <!-- Steering Pointer -->
          <polygon
            :points="pointerPoints(steeringAngle, radius)"
            fill="red"
            stroke="red"
            stroke-width="5"
            stroke-linejoin="round"
          />

          <!-- Tail Pointer -->
          <polygon
            :points="pointerPoints(tailAngle, radius)"
            fill="black"
            stroke="black"
            stroke-width="7"
            stroke-linejoin="round"
          />
        </svg>
      </v-card-text>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { defineProps, computed } from 'vue';

const props = defineProps<{
  data: number[]; // [steering_raw (0–255), tail_raw (0–255)]
  title: string;
  titleColor?: string;
}>();

const svgWidth = 400;
const svgHeight = 150;
const radius = 100;
const cx = svgWidth / 2; // centers everything horizontally

// Convert degrees → Cartesian, shifted so 180°…360° is bottom half.
function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
  const angleRad = (Math.PI * angleDeg) / 180;
  return {
    x: cx + r * Math.cos(angleRad - Math.PI),
    y: cy + r * Math.sin(angleRad - Math.PI),
  };
}

function pointerPoints(angle: number, len: number): string {
  // We draw a small base (width 10px) and a tip at length “len”:
  const angleRad = (Math.PI * angle) / 180 - Math.PI;
  const baseLeft = polarToCartesian(cx, 0, 5, angle - 90);
  const baseRight = polarToCartesian(cx, 0, 5, angle + 90);
  const tip = {
    x: cx + len * Math.cos(angleRad),
    y: 0 + len * Math.sin(angleRad),
  };
  return `${baseLeft.x},${baseLeft.y} ${baseRight.x},${baseRight.y} ${tip.x},${tip.y}`;
}

// Map raw 0–255 so that 255→180° (far left) and 0→360° (far right)
// i.e. bottom semicircle spans from 180° to 360°.
function toBottomSemicircleAngle(raw: number): number {
  const clamped = Math.max(0, Math.min(1023, raw));
  return 180 + ((1023 - clamped) / 1023) * 180;
}

const steeringAngle = computed(() =>
  typeof props.data[0] === 'number' ? toBottomSemicircleAngle(props.data[0]) : 180
);

const tailAngle = computed(() =>
  typeof props.data[1] === 'number' ? toBottomSemicircleAngle(props.data[1]) : 180
);
</script>

<style scoped>
.steering-body {
  background-color: rgb(195, 232, 245);
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
  font-family: var(--zenite-ui-font);
  text-align: center;
}
</style>
