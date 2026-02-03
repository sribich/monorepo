import { useEffect, useLayoutEffect, useRef, useState, type RefObject } from "react"

export interface UseElementSizeOptions {
    watchForResizes?: boolean
}

export interface UseElementSizeResult {
    elementRef: RefObject<HTMLElement>
    elementSize: {
        width: number
        height: number
    }
}

export const useElementSize = (options: UseElementSizeOptions): UseElementSizeResult => {
    const elementRef = useRef<HTMLElement>(null)
    const [elementSize, setElementSize] = useState([0, 0] as [number, number])

    useLayoutEffect(() => {
        if (elementRef.current) {
            const { width, height } = elementRef.current.getBoundingClientRect()

            setElementSize([width, height])
        }
    }, [])

    useEffect(() => {
        if (!options.watchForResizes) {
            return
        }

        let timeout: number

        const onResize = () => {
            timeout = setTimeout(() => {
                if (elementRef.current) {
                    const { width, height } = elementRef.current.getBoundingClientRect()

                    setElementSize(() => [width, height])
                }
            }, 250) as never as number // TypeScript thinks it's a NodeJS timer.
        }

        window.addEventListener("resize", onResize)

        return () => {
            window.removeEventListener("resize", onResize)

            if (timeout) {
                clearTimeout(timeout)
            }
        }
    }, [options.watchForResizes])

    return {
        elementRef,
        elementSize: {
            width: elementSize[0],
            height: elementSize[1],
        },
    }
}
