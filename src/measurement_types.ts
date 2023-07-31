type FormatRule = number | "NonZeroAverage" | "Sum"

export class GenericCardData {
  name: string
  description: string
  units: string
  min: number
  max: number
  precision: number
  data: number[] | boolean[] = []
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

  sum(percentage:boolean=false) {
    if (is_boolean_array(this.data)) {
      return 0
    }

    const sum = this.data.reduce((a: number, b: number) => a + b, 0)

    if (percentage) {
      return (100 * (sum - this.min)) / (this.max - this.min)
    }

    return sum
  }

  avg(percentage:boolean=false) {
    if (is_boolean_array(this.data)) {
      return 0
    }

    const avg = computeAverageWithoutZeros(this.data)

    if (percentage) {
      return (100 * (avg - this.min)) / (this.max - this.min)
    }

    return avg
  }

  item(percentage:boolean=false, index: number) {
    if (is_boolean_array(this.data)) {
      return 0
    }

    if (percentage) {
      return (100 * (this.data[index] - this.min)) / (this.max - this.min)
    }

    return this.data[index]
  }
}

function is_boolean_array(value: unknown): value is boolean[] {
  return Array.isArray(value) && value.every(item => typeof item === "boolean");
}

export const fixate = (value: number, precision: number): string => {
    let str = value.toFixed(precision).substring(0, 5)
    if (str.charAt(str.length - 1) === ".") {
        str = str.substring(0, str.length - 1)
    }

    return str
}

export function computeAverageWithoutZeros(numbers: number[]): number {
    const nonZeroNumbers: number[] = numbers.filter((num) => num !== 0)

    if (nonZeroNumbers.length === 0) {
        return 0 // Avoid division by zero for an array with all zeros or an empty array
    }

    const sum: number = nonZeroNumbers.reduce((acc, num) => acc + num, 0)
    const average: number = sum / nonZeroNumbers.length
    return average
}