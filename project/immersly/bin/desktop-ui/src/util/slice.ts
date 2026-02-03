/**
 * Returns a sized view into an existing array.
 */
export const slice = <T>(array: Array<T>, from: number, to: number) => {
    // Initialize a new sparse array to act as an intermediary for the
    // real array.
    const slicedArray = new Array(Math.max(0, to - from))

    slicedArray[Symbol.iterator] = function* (): IterableIterator<T> {
        for (let i = from; i < to; i++) {
            yield array[i] as T
        }
    }

    return new Proxy(array, {
        get(initialArray: Array<T>, index: string | symbol) {
            if (index === Symbol.iterator) {
                return slicedArray[Symbol.iterator].bind(slicedArray)
            }

            if (typeof index !== "symbol") {
                const sliceIndex = Number.parseInt(index)

                if (!Number.isNaN(sliceIndex)) {
                    return initialArray[from + sliceIndex]
                }
            }

            return slicedArray[index]
        },
        set(initialArray: Array<T>, index: string | symbol, value: T) {
            if (typeof index !== "symbol") {
                const sliceIndex = Number.parseInt(index)

                if (!Number.isNaN(sliceIndex)) {
                    initialArray[from + sliceIndex] = value
                }

                return true
            }

            return false
        },
    })
}

/*
(): IterableIterator<T> => {
        let index = from

        return {
            next: (): IteratorResult<T> => {
                return index < to
                    ? { value: array[index++] as T, done: false }
                    : { value: undefined, done: true }
            },
            [Symbol.iterator]() {
                return this
            }
        }
    }
*/
