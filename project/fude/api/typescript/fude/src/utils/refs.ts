import type { Ref } from "react"

const setRef = <T>(ref: Ref<T> | null | undefined, value: T): (() => void) | void => {
    if (typeof ref === "function") {
        return ref(value)
    } else if (ref) {
        ref.current = value
    }
}

export const mergeRefs = <TRef>(...refs: Array<Ref<TRef> | null | undefined>): Ref<TRef> => {
    if (refs.length === 1 && refs[0]) {
        return refs[0]
    }

    return (value: TRef | null): (() => void) | void => {
        let hasCleanup = false

        const cleanupHandlers = refs.map((ref) => {
            const cleanup = setRef(ref, value)

            hasCleanup ||= typeof cleanup === "function"

            return cleanup
        })

        if (hasCleanup) {
            return () => {
                cleanupHandlers.forEach((cleanup, i) => {
                    if (typeof cleanup === "function") {
                        cleanup()
                    } else {
                        setRef(refs[i], null)
                    }
                })
            }
        }
    }
}
