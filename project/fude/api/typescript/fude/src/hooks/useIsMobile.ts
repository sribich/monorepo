import { useEffect, useState } from "react"

const MOBILE_SCREEN_BREAKPOINT = 768

export const useIsMobileScreen = () => {
    const [isMobile, setIsMobile] = useState<boolean | undefined>(undefined)

    useEffect(() => {
        const query = window.matchMedia(`(max-width: ${MOBILE_SCREEN_BREAKPOINT}px)`)
        const onChange = (event: MediaQueryListEvent) => setIsMobile(event.matches)

        setIsMobile(window.innerWidth < MOBILE_SCREEN_BREAKPOINT)

        query.addEventListener("change", onChange)

        return () => query.removeEventListener("change", onChange)
    }, [])

    if (isMobile === undefined) {
        return window.innerWidth < MOBILE_SCREEN_BREAKPOINT
    }

    return isMobile
}
