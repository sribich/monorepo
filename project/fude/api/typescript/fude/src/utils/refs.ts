import type { ForwardedRef, Ref } from "react"

/**
 * Merges multiple refs into one. Works with both callback and object refs.
 */
export const mergeRefs = <TRef>(...refs: (ForwardedRef<TRef> | undefined)[]): Ref<TRef> => {
    if (refs.length === 1 && refs[0]) {
        return refs[0]
    }

    return (value: TRef | null) => {
        for (const ref of refs) {
            if (typeof ref === "function") {
                ref(value)
            } else if (ref != null) {
                ref.current = value
            }
        }
    }
}
