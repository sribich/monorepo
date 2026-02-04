import { type MutableRefObject, type Ref, useCallback } from "react"

export type PossibleRef<T> = Ref<T> | undefined

const setRef = <T>(ref: PossibleRef<T>, value: T) => {
    if (!ref) {
        return
    }

    if (typeof ref === "function") {
        ref(value)
    } else if (ref) {
        ;(ref as MutableRefObject<T>).current = value
    }
}

export const useComposedRefs = <T>(...refs: PossibleRef<T>[]) => {
    return useCallback((node: T) => refs.forEach((it) => setRef(it, node)), refs)
}
