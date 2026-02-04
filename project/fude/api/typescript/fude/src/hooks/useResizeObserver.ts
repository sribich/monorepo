import { type RefObject, useEffect } from "react"

export interface UseResizeObserverOptions<T> {
    ref: RefObject<T | undefined | null> | undefined
    box?: ResizeObserverBoxOptions
    onResize: () => void
}

export const useResizeObserver = <T extends Element>(options: UseResizeObserverOptions<T>) => {
    const { ref, box, onResize } = options

    useEffect(() => {
        const element = ref?.current

        if (!element) {
            return
        }

        if (typeof window.ResizeObserver === "undefined") {
            window.addEventListener("resize", onResize, false)

            return () => {
                window.removeEventListener("resize", onResize, false)
            }
        }

        const observer = new window.ResizeObserver((entries) => {
            if (entries.length) {
                onResize()
            }
        })

        observer.observe(element, box ? { box } : {})

        return () => {
            if (element) {
                observer.unobserve(element)
            }
        }
    }, [ref, box, onResize])
}
