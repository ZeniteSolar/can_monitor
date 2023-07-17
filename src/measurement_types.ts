
export class MeasurementCardData {
  name: string
  description: string
  units: string
  min: number
  max: number
  precision: number
  value: number = 0.0

  constructor(name: string, description: string, units: string, min: number, max: number, precision: number) {
    this.name = name
    this.description = description
    this.units = units
    this.min = min
    this.max = max
    this.precision = precision
    this.value = 0.0
  }
}
