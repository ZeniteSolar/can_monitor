export interface BoardState {
  label: string;
  value: number;
}

export interface BoardMetric {
  label?: string;
  data: number[];
  units: string[];
}

export enum Orientation {
  VERTICAL = 'VERTICAL',
  HORIZONTAL = 'HORIZONTAL',
}

