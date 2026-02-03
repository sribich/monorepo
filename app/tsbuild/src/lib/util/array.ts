export const arrayIncludes = <T>(array: ReadonlyArray<T>, item: unknown): item is T => {
    return (array as ReadonlyArray<unknown>).includes(item)
}
