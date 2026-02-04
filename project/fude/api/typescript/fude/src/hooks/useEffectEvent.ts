import {
    // @ts-expect-error
    experimental_useEffectEvent,
} from "react"

export const useEffectEvent: <T extends Function>(fn?: T) => T = experimental_useEffectEvent
