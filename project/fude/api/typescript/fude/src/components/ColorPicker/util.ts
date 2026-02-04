import type { ElementRef } from "react"

import type { HsbColor } from "./color"

export const checkerboard = (() => {
    const cache = new Map()

    return (colorA: string, colorB: string, cellSize: number) => {
        const key = `${colorA}-${colorB}-${cellSize}`

        if (!cache.has(key)) {
            const canvas = document.createElement("canvas")

            canvas.width = cellSize * 2
            canvas.height = cellSize * 2

            const context = canvas.getContext("2d")

            if (!context) {
                return undefined
            }

            context.fillStyle = colorA
            context.fillRect(0, 0, canvas.width, canvas.height)

            context.fillStyle = colorB
            context.fillRect(0, 0, cellSize, cellSize)
            context.fillRect(cellSize, cellSize, cellSize * 2, cellSize * 2)

            cache.set(key, canvas.toDataURL())
        }

        return cache.get(key)
    }
})()

export const calculateHsbFromPosition = (
    hsb: HsbColor["value"],
    position: { x: number; y: number },
    container: ElementRef<"div">,
) => {
    const bounds = container.getBoundingClientRect()

    const left = position.x
    const top = position.y

    return {
        h: hsb.h,
        s: left / bounds.width,
        b: 1 - top / bounds.height,
        a: hsb.a,
    }
}
