import { useEffect, useState } from "react"

const MEDIA_QUERY = "(prefers-reduced-motion: no-preference)"

const getInitialState = () => {
    return !window?.matchMedia?.(MEDIA_QUERY).matches
}

export const usePrefersReducedMotion = () => {
    const [value, setValue] = useState(getInitialState)

    useEffect(() => {
        const query = window?.matchMedia?.(MEDIA_QUERY)

        const onChange = (event: MediaQueryListEvent) => {
            setValue(!event.matches)
        }

        query?.addEventListener("change", onChange)

        return () => query?.removeEventListener("change", onChange)
    }, [])

    return value
}
