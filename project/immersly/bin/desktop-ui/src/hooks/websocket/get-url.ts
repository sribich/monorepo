import type { RefObject } from "react"
import { appendQueryParams } from "./socket-io"
import type { Options } from "./types"
import { DEFAULT_RECONNECT_INTERVAL_MS, DEFAULT_RECONNECT_LIMIT } from "./constants"

const waitFor = (duration: number) => new Promise((resolve) => window.setTimeout(resolve, duration))

export const getUrl = async (
    url: string | (() => string | Promise<string>),
    optionsRef: RefObject<Options>,
    retriedAttempts = 0,
): Promise<string | null> => {
    let parsedUrl: string

    if (typeof url === "function") {
        try {
            parsedUrl = await url()
        } catch (e) {
            if (optionsRef.current.retryOnError) {
                const reconnectLimit =
                    optionsRef.current.reconnectAttempts ?? DEFAULT_RECONNECT_LIMIT

                if (retriedAttempts < reconnectLimit) {
                    const nextReconnectInterval =
                        typeof optionsRef.current.reconnectInterval === "function"
                            ? optionsRef.current.reconnectInterval(retriedAttempts)
                            : optionsRef.current.reconnectInterval

                    await waitFor(nextReconnectInterval ?? DEFAULT_RECONNECT_INTERVAL_MS)
                    return getUrl(url, optionsRef, retriedAttempts + 1)
                }

                optionsRef.current.onReconnectStop?.(retriedAttempts)
                return null
            }

            return null
        }
    } else {
        parsedUrl = url
    }

    const parsedWithQueryParams = optionsRef.current.queryParams
        ? appendQueryParams(parsedUrl, optionsRef.current.queryParams)
        : parsedUrl

    return parsedWithQueryParams
}
