export const isInteger = (item: unknown): item is number => {
    return Number.isInteger(item)
}

export const isIntegerInRange = (item: unknown, min: number, max: number): item is number => {
    return typeof item === "number" && Number.isInteger(item) && min <= item && item <= max
}
