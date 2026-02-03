import { SOCKET_IO_PING_INTERVAL, SOCKET_IO_PATH, SOCKET_IO_PING_CODE } from "./constants"
import type { QueryParams, SendMessage } from "./types"

export const appendQueryParams = (url: string, params: QueryParams = {}): string => {
    const hasParamsRegex = /\?([\w]+=[\w]+)/
    const alreadyHasParams = hasParamsRegex.test(url)

    const stringified = `${Object.entries(params)
        .reduce((next, [key, value]) => {
            return next + `${key}=${value}&`
        }, "")
        .slice(0, -1)}`

    return `${url}${alreadyHasParams ? "&" : "?"}${stringified}`
}

export const setUpSocketIOPing = (sendMessage: SendMessage, interval = SOCKET_IO_PING_INTERVAL) => {
    const ping = () => sendMessage(SOCKET_IO_PING_CODE)

    return window.setInterval(ping, interval)
}
