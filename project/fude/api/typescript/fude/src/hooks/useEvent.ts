import type { RefObject, UIEvent } from "react"

import { useEffect } from "react"
import { useEffectEvent } from "./useEffectEvent"

export const useEvent = <K extends keyof GlobalEventHandlersEventMap>(
    ref: RefObject<EventTarget>,
    event: K | (string & {}),
    handler?: (this: Document, event: GlobalEventHandlersEventMap[K]) => unknown,
    options?: boolean | AddEventListenerOptions,
): void => {
    const handleEvent = useEffectEvent(handler) as EventListener
    const isDisabled = !handler

    useEffect(() => {
        if (isDisabled || !ref.current) {
            return
        }

        const element = ref.current

        element.addEventListener(event, handleEvent, options)

        return () => {
            element.removeEventListener(event, handleEvent)
        }
    }, [ref, event, options, isDisabled])
}
