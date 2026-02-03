import { useEffect, useRef, useState, useCallback } from "react"
import { ReadyState } from "./constants"
import { createOrJoinSocket } from "./create-or-join"
import { getUrl } from "./get-url"
import websocketWrapper from "./proxy"
import type { Options, SendMessage, WebSocketHook } from "./types"
import { assertIsWebSocket } from "./util"

export const useWebSocket = <TSend, TRecv, T = unknown>(
    url: string | (() => string | Promise<string>),
    options: Options<TRecv> = {},
): WebSocketHook<TSend, TRecv, T> => {
    const [readyState, setReadyState] = useState<ReadyState>(ReadyState.CONNECTING)

    const webSocketRef = useRef<WebSocket | null>(null)
    const startRef = useRef<() => void>(() => void 0)
    const reconnectCount = useRef<number>(0)
    const messageQueue = useRef<TSend[]>([])
    const webSocketProxy = useRef<WebSocket | null>(null)
    const optionsCache = useRef<Options>(options)
    optionsCache.current = options

    const stringifiedQueryParams = options.queryParams ? JSON.stringify(options.queryParams) : null

    const sendMessage: SendMessage<TSend> = useCallback((message) => {
        if (webSocketRef.current?.readyState === ReadyState.OPEN) {
            assertIsWebSocket(webSocketRef.current, optionsCache.current.skipAssert)
            webSocketRef.current.send(JSON.stringify(message))
        } else {
            messageQueue.current.push(message)
        }
    }, [])

    const getWebSocket = useCallback(() => {
        if (optionsCache.current.share !== true) {
            return webSocketRef.current
        }

        if (webSocketProxy.current === null && webSocketRef.current) {
            assertIsWebSocket(webSocketRef.current, optionsCache.current.skipAssert)
            webSocketProxy.current = websocketWrapper(webSocketRef.current, startRef)
        }

        return webSocketProxy.current
    }, [])

    useEffect(() => {
        let isUnmounted = false
        let removeListeners: () => void

        const start = async () => {
            const realUrl = await getUrl(url, optionsCache)

            if (!realUrl) {
                console.error("Failed to obtain a WebSocket URL.")

                return setReadyState(ReadyState.CLOSED)
            }

            const protectedSetReadyState = (state: ReadyState) => {
                if (!isUnmounted) {
                    setReadyState(state)
                }
            }

            if (!isUnmounted) {
                removeListeners = createOrJoinSocket(
                    webSocketRef,
                    realUrl,
                    protectedSetReadyState,
                    optionsCache,
                    startRef,
                    reconnectCount,
                    sendMessage,
                )
            }
        }

        startRef.current = () => {
            if (!isUnmounted) {
                if (webSocketProxy.current) webSocketProxy.current = null
                removeListeners?.()
                start()
            }
        }

        start()

        return () => {
            isUnmounted = true

            if (webSocketProxy.current) webSocketProxy.current = null
            removeListeners?.()
        }
    }, [url, stringifiedQueryParams, sendMessage])

    useEffect(() => {
        if (readyState === ReadyState.OPEN) {
            for (const message of messageQueue.current.splice(0)) {
                sendMessage(message)
            }
        }
    }, [readyState, sendMessage])

    return {
        sendMessage,
        readyState,
        getWebSocket,
    }
}
