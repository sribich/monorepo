import { useState, type Key, type MouseEvent } from "react"
import type { Ripple } from "./Ripple.js"
import { usePrefersReducedMotion } from "../../hooks/usePrefersReducedMotion.js"

let rippleId = 0

export interface RippleHook {
    rippleProps: {
        onClick: (event: MouseEvent<HTMLElement>) => void
    }
    ripples: Array<Ripple.Ripple>
    clearRipple: (id: number) => void
}

export const useRipple = (disableAnimations?: boolean): RippleHook => {
    const reduceMotion = usePrefersReducedMotion() || disableAnimations

    const [ripples, setRipples] = useState<Array<Ripple.Ripple>>([])

    const rippleProps = {
        onClick: (event: MouseEvent<HTMLElement>) => {
            if (reduceMotion) {
                return
            }

            const target = event.currentTarget
            const size = Math.max(target.clientWidth, target.clientHeight)
            const rect = target.getBoundingClientRect()

            setRipples((prevRipples) => [
                ...prevRipples,
                {
                    key: rippleId++,
                    size,
                    x: event.clientX - rect.x - size / 2,
                    y: event.clientY - rect.y - size / 2,
                },
            ])
        },
    }

    const clearRipple = (key: Key) => {
        setRipples((prevState) => prevState.filter((ripple) => ripple.key > key))
    }

    return { rippleProps, ripples, clearRipple }
}
