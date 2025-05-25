import { reactive } from 'vue';
import type { GenericCardData } from '@/measurement_types';

export const measurementCards = reactive<Record<string, GenericCardData>>({});
