import { createFileRoute } from "@tanstack/react-router"
import { useState, type ReactNode } from "react"
import type { ClientMessages, ServerMessages } from "../../generated/rpc-client/websocket"
import { useWebSocket } from "../../hooks/websocket/use-websocket"

export const Route = createFileRoute("/_deactivated/immerse")({
    component: () => <ImmerseRoute />,
})

const ImmerseRoute = () => {
    const [messages, setMessages] = useState([] as ReactNode[])

    const { sendMessage } = useWebSocket<ClientMessages, ServerMessages>("ws://127.0.0.1:7057/ws", {
        onParsedMessage: (data) => {
            if (data.kind === "Subtitle") {
                const result = data.content.map((item) => {
                    switch (item.kind) {
                        case "Raw":
                            return <span>{item.content}</span>
                        case "Tagged":
                            return <span>{item.content.surface}</span>
                    }
                })
                setMessages((prev) => [...prev, <p>{result}</p>])
            }
        },
    })

    return <div>{messages}</div>
}
