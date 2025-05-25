// in types/index.ts or wherever you define shared types
export interface BoardState {
  label: string;
  value: number;
  description?: string; // ✅ add this if not present
}

export interface BoardMetric {
  label?: string;
  data: number[];
  units: string[];
}

export interface BoardBoolean {
  label: string;
  value: boolean;
}

export enum Orientation {
  VERTICAL = 'VERTICAL',
  HORIZONTAL = 'HORIZONTAL',
}