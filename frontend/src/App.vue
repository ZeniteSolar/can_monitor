<template>
  <Suspense>
    <MeasurementsDashboard />
  </Suspense>
</template>

<script setup lang="ts">
import MeasurementsDashboard from "./components/MeasurementsDashboard.vue";
</script>

<style>
/* Fundo geral */
body, #app {
  background-color: white !important;
  color: black !important;
}

/* Fundo dos cards */
.card, .v-card {
  background-color: white !important;
  color: black !important;
  border: 1px solid #ccc; /* Opcional: para dar contorno */
  border-radius: 8px; /* Opcional: bordas arredondadas */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); /* Opcional: sombra suave */
}
</style>
