import { useLayoutEffect as reactLayoutEffect } from "react"

/**
 * Perform a no-op when rendering on the server to prevent react from
 * emitting a warning about it.
 */
export const useLayoutEffect: typeof reactLayoutEffect =
    typeof document !== "undefined" ? reactLayoutEffect : () => {}
