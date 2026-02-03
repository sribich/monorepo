import type { RefObject } from "react"
import type { ReadyState } from "./constants"

export interface QueryParams {
    [key: string]: string | number
}

export interface Options<TRecv> {
    onParsedMessage?: (message: TRecv) => void

    queryParams?: QueryParams
    protocols?: string | string[]
    share?: boolean
    onOpen?: (event: WebSocketEventMap["open"]) => void
    onClose?: (event: WebSocketEventMap["close"]) => void
    onMessage?: (event: WebSocketEventMap["message"]) => void
    onError?: (event: WebSocketEventMap["error"]) => void
    onReconnectStop?: (numAttempts: number) => void
    shouldReconnect?: (event: WebSocketEventMap["close"]) => boolean
    reconnectInterval?: number | ((lastAttemptNumber: number) => number)
    reconnectAttempts?: number
    filter?: (message: WebSocketEventMap["message"]) => boolean
    retryOnError?: boolean
    skipAssert?: boolean
    heartbeat?: boolean | HeartbeatOptions
}

export type HeartbeatOptions = {
    message?: "ping" | "pong" | string | (() => string)
    returnMessage?: "ping" | "pong" | string
    timeout?: number
    interval?: number
}

export type SendMessage<T> = (message: T) => void

export type Subscriber<T = WebSocketEventMap["message"]> = {
    setReadyState: (readyState: ReadyState) => void
    optionsRef: RefObject<Options>
    reconnectCount: RefObject<number>
    reconnect: RefObject<() => void>
}

export interface WebSocketHook<TSend, TRecv, T = unknown, P = WebSocketEventMap["message"] | null> {
    sendMessage: SendMessage<TSend>
    readyState: ReadyState
    getWebSocket: () => WebSocket | null
}
