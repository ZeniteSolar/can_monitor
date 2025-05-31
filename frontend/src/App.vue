<template>
  <div id="app">
    <Suspense>
      <MeasurementsDashboard />
    </Suspense>
  </div>
</template>

<script setup lang="ts">
import MeasurementsDashboard from "./components/MeasurementsDashboard.vue";

// Read VITE_FLIP_SCREEN from .env
</script>

<style>
:root {
  --zenite-background: #ffffff;
  --zenite-on-background: #000000;
  --zenite-surface: #ffffff;
  --zenite-primary: #eb8b04;
  --zenite-primary-darken-1: #c56f03;
  --zenite-secondary: #0493ba;
  --zenite-secondary-darken-1: #036c8a;
  --zenite-terciary: #1e1e1e;
  --zenite-error: #ff5370;
  --zenite-info: #89ddff;
  --zenite-success: #c3e88d;
  --zenite-warning: #f78c6c;
  --zenite-ui-font: 'Inter', 'Roboto', 'Arial', sans-serif;
  --zenite-data-font: 'JetBrains Mono', monospace;
}

/* Global app styling */
body, #app {
  background-color: var(--zenite-background);
  color: var(--zenite-on-background) !important;
  font-family: var(--zenite-ui-font);
}

.card, .v-card {
  background-color: var(--zenite-background) !important;
  color: var(--zenite-on-background) !important;
  border: 1px solid #ccc;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.v-card-title.bg-primary {
  color: black !important;
}

/* Monospaced text for data readouts */
.text-mono {
  font-family: var(--zenite-data-font);
}

/* Optional flipping for deployment (based on env flag) */
#app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

</style>