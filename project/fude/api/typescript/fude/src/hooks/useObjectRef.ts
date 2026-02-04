import { useMemo, useRef, type RefObject } from "react"

export const useObjectRef = <T>(
    forwardedRef?: ((instance: T | null) => void) | RefObject<T | null> | null,
): RefObject<T> => {
    const objRef = useRef<T>(undefined)

    return useMemo(
        () => ({
            get current() {
                return objRef.current as T
            },
            set current(value: T) {
                objRef.current = value

                if (typeof forwardedRef === "function") {
                    forwardedRef(value)
                } else if (forwardedRef) {
                    forwardedRef.current = value
                }
            },
        }),
        [forwardedRef],
    )
}
