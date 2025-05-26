<template>
  <div class="zenite-header">
    <div class="header-grid">
      <!-- Left Logo -->
      <div class="logo-container" style="justify-content: flex-start">
        <img v-if="leftLogo2" :src="leftLogo2" alt="Left Logo" class="logo-img" />
        <img v-if="leftLogo3" :src="leftLogo3" alt="Left Logo" class="logo-img" />
        <img v-if="leftLogo1" :src="leftLogo1" alt="Left Logo" class="logo-img" />
      </div>


      <!-- Center Title -->
      <div class="zenite-title text-primary">ZÊNITE SOLAR</div>

      <!-- Right Side: Sponsor label and logos -->
      <div class="right-section">
        <div class="sponsor-title text-primary">Apoio</div>
        <div class="logo-container" style="justify-content: flex-end">
          <img v-if="rightLogo1" :src="rightLogo1" alt="Right Logo" class="logo-img" />
          <img v-if="rightLogo2" :src="rightLogo2" alt="Right Logo" class="logo-img" />
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

const props = defineProps<{
  leftLogo1?: string;
  leftLogo2?: string;
  leftLogo3?: string;
  rightLogo1?: string;
  rightLogo2?: string;
  rightLogo3?: string;
  rightLogo4?: string;
}>();
</script>

<style scoped>
.zenite-header {
  background-color: black;
  height: 70px;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #000;
  border-radius: 10px;
}

.header-grid {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  width: 100%;
  padding: 0 16px;
}

.logo-container {
  background-color: black;
  display: flex;
  gap: 20px;
  /* spacing between logos */
  align-items: center;
}

.logo-img {
  background-color: white;
  border: 1px solid #ccc;
  border-radius: 8px;
  height: 60px;
  max-width: 100px;
  object-fit: contain;
  ;
}

.right-section {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 5px;
}

.sponsor-title {
  font-family: var(--zenite-data-font);
  font-size: 1.8rem;
  /* font-weight: bold; */
  font-style:italic;
  white-space: nowrap;
  
}


.zenite-title {
  font-family: var(--zenite-data-font);
  font-size: 2.8rem;
  /* font-weight: bold; */
  text-align: center;
  white-space: nowrap;
}
</style>
