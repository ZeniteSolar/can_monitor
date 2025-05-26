<template>
  <v-app-bar app flat height="80" class="zenite-header d-flex align-center justify-center">
    <v-container class="d-flex align-center justify-center fill-height px-4">
      <v-row class="align-center justify-center fill-height" no-gutters>
        <!-- Optional Left Logo -->
        <v-col cols="2" class="d-flex justify-center">
          <img v-if="leftLogo" :src="leftLogo" alt="Left Logo" class="logo-img" />
        </v-col>

        <!-- Title -->
        <v-col cols="8" class="d-flex align-center justify-center">
          <div class="zenite-title text-primary">ZÊNITE SOLAR</div>
        </v-col>

        <!-- Optional Right Logo -->
        <v-col cols="2" class="d-flex justify-center">
          <img v-if="rightLogo" :src="rightLogo" alt="Right Logo" class="logo-img" />
        </v-col>
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

const props = defineProps<{
  leftLogo?: string;
  rightLogo?: string;
}>();
</script>

<style scoped>
.zenite-header {
  background-color: black !important;
  border-bottom: 1px solid #000 !important;
  z-index: 1000;
}

.logo-img {
  height: 60px;
  object-fit: contain;
}

.zenite-title {
  font-family: var(--zenite-ui-font);
  font-size: 3rem;
  font-weight: bold;
  text-align: center;
}
</style>
