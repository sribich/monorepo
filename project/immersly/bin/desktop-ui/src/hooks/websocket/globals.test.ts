import { sharedWebSockets, resetWebSockets } from "./globals"

const FIRST_URL = "ws://localhost:1234"
const SECOND_URL = "ws://localhost:4321"

const websocket1 = {} as WebSocket
const websocket2 = {} as WebSocket

beforeEach(() => {
    resetWebSockets()
})

test("resetWebsockets removes subscribers only for a specific URL", () => {
    sharedWebSockets[FIRST_URL] = websocket1
    sharedWebSockets[SECOND_URL] = websocket2
    expect(Object.values(sharedWebSockets)).toHaveLength(2)

    resetWebSockets(FIRST_URL)

    expect(sharedWebSockets[FIRST_URL]).toBeUndefined()
    expect(sharedWebSockets[SECOND_URL]).not.toBeUndefined()
})

test("resetWebsockets removes all subscribers when URL is not set", () => {
    sharedWebSockets[FIRST_URL] = websocket1
    sharedWebSockets[SECOND_URL] = websocket2
    expect(Object.values(sharedWebSockets)).toHaveLength(2)

    resetWebSockets()

    expect(Object.values(sharedWebSockets)).toHaveLength(0)
})
