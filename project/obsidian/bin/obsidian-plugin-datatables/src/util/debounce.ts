import { useEffect, useMemo } from "react"

export interface DebounceArgs<T extends unknown[]> {
    /**
     * The function to debounce. This function is called a maximum of
     * one time per debounce period.
     */
    onComplete: (...args: T) => void | Promise<void>
    /**
     * A function called when the debounced function has already been
     * called and it is waiting for the debounce period to end.
     *
     * This is useful for cases where you want to perform some action
     * alongside the debounced function, but where it is not necessary
     * for the debounced function to actually execute.
     *
     * For example, when updating the datatables `schema` file, we keep
     * the datatable data in memory and want to emit an event any time
     * a change comes in. However, we don't want to write the changes
     * after every single change, so we debounce the write operation.
     */
    onDebounce: (...args: T) => void | Promise<void>
}

export const debounce = <T extends unknown[]>(args: DebounceArgs<T>, delay: number) => {
    let timer: NodeJS.Timeout | undefined

    let requeue = false
    let inAsyncScope = false

    const debouncer = async (...funcArgs: T) => {
        if (inAsyncScope) {
            requeue = true
            return
        }

        clearTimeout(timer)

        timer = setTimeout(async () => {
            inAsyncScope = true

            await args.onComplete(...funcArgs)

            inAsyncScope = false

            if (requeue) {
                requeue = false
                await debouncer(...funcArgs)
            }
        }, delay)

        args.onDebounce(...funcArgs)
    }

    debouncer.cancel = () => {
        clearTimeout(timer)
    }

    return debouncer
}

export const useDebounce = <T extends unknown[]>(
    func: (...args: T) => void | Promise<void>,
    delay: number,
) => {
    const debouncer = useMemo(() => {
        return debounce(
            {
                onComplete: func,
                onDebounce: () => {
                    void 0
                },
            },
            delay,
        )
    }, [func, delay])

    useEffect(() => {
        return () => debouncer.cancel()
    }, [debouncer])

    return debouncer
}
