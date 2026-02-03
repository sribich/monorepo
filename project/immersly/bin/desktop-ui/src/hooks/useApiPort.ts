import { createContext, useEffect, useState } from "react"

export const ApiHostContext = createContext({ host: "http://127.0.0.1" })

export const useApiPort = () => {
    const [port, setPort] = useState(0)

    useEffect(() => {
        ;(async () => {
            const port = window.__TAURI__ ? await __TAURI__.core.invoke("get_api_port") : 7057

            setPort(port)
        })()
    }, [])

    return {
        port,
        host: `http://127.0.0.1:${port}`,
        isLoading: port === 0,
    }
}
