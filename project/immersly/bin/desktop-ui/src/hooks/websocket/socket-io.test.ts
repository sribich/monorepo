import { appendQueryParams } from "./socket-io"

const URL = "ws://localhost:1234"

test("appendQueryParams adds query params from an object to a given url", () => {
    const queryParams = { type: "user", id: 5 }
    const wsUrl = appendQueryParams(URL, queryParams)
    expect(wsUrl).toEqual(`${URL}?type=user&id=5`)
})

test("appendQueryParams properly adds query params to a url that already contains query params", () => {
    const queryParams = { type: "user", id: 5 }
    let wsUrl = appendQueryParams(URL, queryParams)
    wsUrl = appendQueryParams(wsUrl, { name: "bob" })

    expect(wsUrl).toEqual(`${URL}?type=user&id=5&name=bob`)
})
