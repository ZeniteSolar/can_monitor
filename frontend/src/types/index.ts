export interface BoardState {
  label: string;
  value: number;
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

