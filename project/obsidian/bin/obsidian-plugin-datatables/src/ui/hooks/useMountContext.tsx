import { type Context, useContext } from "react"
import { useSnapshot } from "valtio"

import { MountContext } from "../Mount"

interface ValidContext {
    proxy: unknown
}

export const useMountContext = <T extends ValidContext>(): T & { proxyMut: T["proxy"] } => {
    const { proxy, ...context } = useContext<T>(MountContext as Context<T>)

    if (!proxy) {
        throw new Error("MountContext must contain a valtio proxy as the 'proxy' key.")
    }

    return {
        ...context,
        proxy: useSnapshot(proxy),
        proxyMut: proxy as T["proxy"],
    } as T & { proxyMut: T["proxy"] }
}
