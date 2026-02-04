export const objectToPrettyJson = (object: Record<string, unknown>): string => {
    return JSON.stringify(object, undefined, 2)
}
