

type FormatRule = number | "NonZeroAverage" | "Sum"

export class MeasurementCardData {
  name: string
  description: string
  units: string
  min: number
  max: number
  precision: number
  data: number[] = [0.0]
  rule: FormatRule

  constructor(name: string, description: string, units: string, min: number, max: number, precision: number, rule: FormatRule) {
    this.name = name
    this.description = description
    this.units = units
    this.min = min
    this.max = max
    this.precision = precision
    this.rule = rule
  }
}
