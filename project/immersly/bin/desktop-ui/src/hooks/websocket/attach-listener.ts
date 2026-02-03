import type { RefObject } from "react"
import { heartbeat } from "./heartbeat"
import { DEFAULT_RECONNECT_LIMIT, DEFAULT_RECONNECT_INTERVAL_MS, ReadyState } from "./constants"
import type { Options, SendMessage } from "./types"
import { assertIsWebSocket } from "./util"

export interface Setters {
    setReadyState: (readyState: ReadyState) => void
}

const bindMessageHandler = (webSocketInstance: WebSocket, optionsRef: RefObject<Options>) => {
    let heartbeatCb: () => void

    if (optionsRef.current.heartbeat && webSocketInstance instanceof WebSocket) {
        const heartbeatOptions =
            typeof optionsRef.current.heartbeat === "boolean"
                ? undefined
                : optionsRef.current.heartbeat
        heartbeatCb = heartbeat(webSocketInstance, heartbeatOptions)
    }

    webSocketInstance.onmessage = async (message: WebSocketEventMap["message"]) => {
        heartbeatCb?.()

        optionsRef.current.onMessage && optionsRef.current.onMessage(message)

        if (optionsRef.current.onParsedMessage) {
            const data = message.data

            if (typeof data === "string") {
                optionsRef.current.onParsedMessage(JSON.parse(data))
            } else {
                optionsRef.current.onParsedMessage(JSON.parse(await data.text()))
            }
        }

        if (
            typeof optionsRef.current.filter === "function" &&
            optionsRef.current.filter(message) !== true
        ) {
            return
        }
        if (
            optionsRef.current.heartbeat &&
            typeof optionsRef.current.heartbeat !== "boolean" &&
            optionsRef.current.heartbeat?.returnMessage === message.data
        )
            return
    }
}

const bindOpenHandler = (
    webSocketInstance: WebSocket,
    optionsRef: RefObject<Options>,
    setReadyState: Setters["setReadyState"],
    reconnectCount: RefObject<number>,
) => {
    webSocketInstance.onopen = (event: WebSocketEventMap["open"]) => {
        optionsRef.current.onOpen && optionsRef.current.onOpen(event)
        reconnectCount.current = 0
        setReadyState(ReadyState.OPEN)
    }
}

const bindCloseHandler = (
    webSocketInstance: WebSocket,
    optionsRef: RefObject<Options>,
    setReadyState: Setters["setReadyState"],
    reconnect: () => void,
    reconnectCount: RefObject<number>,
) => {
    assertIsWebSocket(webSocketInstance, optionsRef.current.skipAssert)
    let reconnectTimeout: number

    webSocketInstance.onclose = (event: WebSocketEventMap["close"]) => {
        optionsRef.current.onClose && optionsRef.current.onClose(event)
        setReadyState(ReadyState.CLOSED)
        if (optionsRef.current.shouldReconnect && optionsRef.current.shouldReconnect(event)) {
            const reconnectAttempts =
                optionsRef.current.reconnectAttempts ?? DEFAULT_RECONNECT_LIMIT
            if (reconnectCount.current < reconnectAttempts) {
                const nextReconnectInterval =
                    typeof optionsRef.current.reconnectInterval === "function"
                        ? optionsRef.current.reconnectInterval(reconnectCount.current)
                        : optionsRef.current.reconnectInterval

                reconnectTimeout = window.setTimeout(() => {
                    reconnectCount.current++
                    reconnect()
                }, nextReconnectInterval ?? DEFAULT_RECONNECT_INTERVAL_MS)
            } else {
                optionsRef.current.onReconnectStop &&
                    optionsRef.current.onReconnectStop(reconnectAttempts)
                console.warn(`Max reconnect attempts of ${reconnectAttempts} exceeded`)
            }
        }
    }

    return () => reconnectTimeout && window.clearTimeout(reconnectTimeout)
}

const bindErrorHandler = (
    webSocketInstance: WebSocket,
    optionsRef: RefObject<Options>,
    setReadyState: Setters["setReadyState"],
    reconnect: () => void,
    reconnectCount: RefObject<number>,
) => {
    let reconnectTimeout: number

    webSocketInstance.onerror = (error: WebSocketEventMap["error"]) => {
        optionsRef.current.onError && optionsRef.current.onError(error)

        if (optionsRef.current.retryOnError) {
            if (
                reconnectCount.current <
                (optionsRef.current.reconnectAttempts ?? DEFAULT_RECONNECT_LIMIT)
            ) {
                const nextReconnectInterval =
                    typeof optionsRef.current.reconnectInterval === "function"
                        ? optionsRef.current.reconnectInterval(reconnectCount.current)
                        : optionsRef.current.reconnectInterval

                reconnectTimeout = window.setTimeout(() => {
                    reconnectCount.current++
                    reconnect()
                }, nextReconnectInterval ?? DEFAULT_RECONNECT_INTERVAL_MS)
            } else {
                optionsRef.current.onReconnectStop &&
                    optionsRef.current.onReconnectStop(
                        optionsRef.current.reconnectAttempts as number,
                    )
                console.warn(
                    `Max reconnect attempts of ${optionsRef.current.reconnectAttempts} exceeded`,
                )
            }
        }
    }

    return () => reconnectTimeout && window.clearTimeout(reconnectTimeout)
}

export const attachListeners = (
    webSocketInstance: WebSocket,
    setters: Setters,
    optionsRef: RefObject<Options>,
    reconnect: () => void,
    reconnectCount: RefObject<number>,
    sendMessage: SendMessage,
): (() => void) => {
    const { setReadyState } = setters

    let interval: number
    let cancelReconnectOnClose: () => void
    let cancelReconnectOnError: () => void

    bindMessageHandler(webSocketInstance, optionsRef)

    bindOpenHandler(webSocketInstance, optionsRef, setReadyState, reconnectCount)

    cancelReconnectOnClose = bindCloseHandler(
        webSocketInstance,
        optionsRef,
        setReadyState,
        reconnect,
        reconnectCount,
    )

    cancelReconnectOnError = bindErrorHandler(
        webSocketInstance,
        optionsRef,
        setReadyState,
        reconnect,
        reconnectCount,
    )

    return () => {
        setReadyState(ReadyState.CLOSING)
        cancelReconnectOnClose()
        cancelReconnectOnError()
        webSocketInstance.close()
        if (interval) clearInterval(interval)
    }
}
