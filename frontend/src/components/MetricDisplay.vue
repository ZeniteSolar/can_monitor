<template>
  <v-card-text v-if="showLabel" align="center" class="font-weight-bold text-caption pa-0 ma-0">{{ label }}</v-card-text>
  <v-progress-linear ref="progressLinear" :model-value="percentage" height="35px" class="pa-0 my-1">
    <template v-slot:default>
      <div class="text-container" ref="textContainer">
        <p class="text-caption invert ma-0" :style="textStyle">
          {{ formattedValue }} {{ units }}
        </p>
      </div>
    </template>
  </v-progress-linear>
</template>

<script setup>
import { computed, ref, onMounted, watch, nextTick, onBeforeUnmount } from 'vue';
import { format } from 'numerable';

// Helper function for debouncing
const debounce = (func, wait) => {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(this, args), wait);
  };
};

const props = defineProps({
  label: String,
  value: [Number, String],
  units: String,
  max: Number,
  min: Number,
  showLabel: {
    type: Boolean,
    default: true
  }
});

const progressLinear = ref(null);
const textContainer = ref(null);
const textStyle = ref({ fontSize: '14px !important' });

const percentage = computed(() => {
  if (typeof props.value === 'number')
    return (100 * (props.value - props.min)) / (props.max - props.min);
  return props.value;
});

const formattedValue = computed(() => format(props.value, '00.0'));

const adjustFontSize = async () => {
  await nextTick(); // wait for DOM update
  if (progressLinear.value && textContainer.value) {
    const containerWidth = progressLinear.value.$el.clientWidth;
    const containerHeight = progressLinear.value.$el.clientHeight;
    let fontSize = 14; // initial font size
    const minFontSize = 8;
    const maxFontSize = 30; // maximum font size to increase to

    // Decrease font size if text is overflowing width or height
    while ((textContainer.value.scrollWidth > containerWidth || textContainer.value.scrollHeight > containerHeight) && fontSize > minFontSize) {
      fontSize -= 1;
      textStyle.value = { fontSize: `${fontSize}px !important` };
      await nextTick(); // wait for DOM update
    }

    // Increase font size if there is more space
    while ((textContainer.value.scrollWidth < containerWidth && textContainer.value.scrollHeight < containerHeight) && fontSize < maxFontSize) {
      fontSize += 1;
      textStyle.value = { fontSize: `${fontSize}px !important` };
      await nextTick(); // wait for DOM update

      // Stop if increasing the font size causes overflow
      if (textContainer.value.scrollWidth > containerWidth || textContainer.value.scrollHeight > containerHeight) {
        fontSize -= 1;
        textStyle.value = { fontSize: `${fontSize}px !important` };
        break;
      }
    }
  }
};

// Debounced version of adjustFontSize with 250ms interval
const debouncedAdjustFontSize = debounce(adjustFontSize, 1000);

onMounted(() => {
  adjustFontSize();
  const intervalId = setInterval(adjustFontSize, 10000);

  onBeforeUnmount(() => {
    clearInterval(intervalId);
  });
});

watch(() => props.value, debouncedAdjustFontSize);
</script>

<style scoped>
.invert {
  mix-blend-mode: difference;
}
.text-container {
  display: flex;
  justify-content: center;
  align-items: center;
  white-space: nowrap;
  overflow: hidden;
}
</style>
