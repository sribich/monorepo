import { Button } from "@sribich/fude"
import { useMutation, useQuery } from "@tanstack/react-query"
import { Outlet, createFileRoute } from "@tanstack/react-router"
import { useEffect, useRef } from "react"

export const Route = createFileRoute("/_deactivated/anki")({
    component: () => {
        const result = useMutation({
            mutationKey: [],
            mutationFn: () => foo({ name: "Bob", age: "30" }),
        })

        console.log(result)

        const clicked = () => {
            result.mutateAsync()
        }

        return (
            <div>
                <Render freqs={result?.data?.freqs ?? []} />
                {result.isPending ? <div>TRUE</div> : <div>FALSE</div>}
                <Button onPress={clicked}>Hello World!</Button>
            </div>
        )
    },
})

export const NOTE_STRINGS: string[] = [
    "C",
    "D♭",
    "D",
    "E♭",
    "E",
    "F",
    "G♭",
    "G",
    "A♭",
    "A",
    "B♭",
    "B",
]

export const OCTAVE_COLORS: [number, number, number][] = [
    [121, 85, 72], // brown
    [158, 158, 158], // grey500
    [96, 125, 139], // bluegrey
    [76, 175, 80], // green
    [244, 67, 54], // red
    [33, 150, 243], // blue
    [0, 150, 136], //teal
    [255, 235, 59], // yellow
    [0, 188, 212], // cyan
]

const Render = ({ freqs = [] }) => {
    const canvasRef = useRef<HTMLCanvasElement>()

    useEffect(() => {
        console.log("here?")
        const ref = canvasRef.current

        if (!ref) {
            console.log("noref")
            return
        }

        const context = ref.getContext("2d")

        if (!context) {
            console.log("nocontext")
            return
        }

        const w = ref.width
        const h = ref.height
        context.fillStyle = "#efefef"
        context.clearRect(0, 0, w, h)
        context.fillRect(0, 0, w, h)

        /*
        for (let i = 0; i < NOTE_STRINGS.length; ++i) {
            // const y = this.scaleY(i)
            context.fillStyle = this.highlight + "55"
            context.fillRect(0, y, w, 1)
            context.fillStyle = this.highlight
            context.font = "14px Sans"
            context.fillText(NOTE_STRINGS[i], this.scaleX(0) + 20, y - 2)
            context.fillText(NOTE_STRINGS[i], 20, y - 2)
        }

        this.bgContext.fillStyle = this.highlight + "55"
        this.bgContext.fillRect(this.scaleX(0), 0, 1, h)
             */

        let averageFreq = freqs.filter((it) => !!it)
        // .map((it) => Number.parseFloat(it.split(" ")[0]))
        averageFreq = averageFreq.reduce((acc, item) => acc + item, 0) / averageFreq.length

        console.log(averageFreq)

        let lastPoint = null
        for (let i = 0; i < freqs.length; i++) {
            const posX = (w / freqs.length) * i
            const yMid = h / 2

            const item = freqs[i]

            if (!item) {
                lastPoint = null
                continue
            }

            const freq = item
            const clamp = (num, min, max) => Math.min(Math.max(num, min), max)
            const normalize = (num, min_num, max_num, min, max) =>
                (max - min) * ((num - min_num) / (max_num - min_num)) + min

            const CLAMP_RANGE = 100
            // 0,0 is top left of canvas. We need to negate abs to flip it from the assumed bottom left
            const abs = -clamp(freq - averageFreq, -CLAMP_RANGE, CLAMP_RANGE)

            const yPos = yMid + normalize(abs, -CLAMP_RANGE, CLAMP_RANGE, -1, 1) * yMid // (abs / 100) * yMid // -50 -> -1    50 -> 1

            // context.fillStyle = `rgba(${color[0]}, ${color[1]}, ${color[2]}, ${clarity * 0.5})`;

            /*
            context.fillStyle = `#000000`
            context.beginPath()
            context.arc(posX, yPos, 1, 0, Math.PI * 2)
            context.fill()
                 */
            if (lastPoint) {
                context.beginPath()
                context.moveTo(lastPoint[0], lastPoint[1])
                context.lineTo(posX, yPos)
                context.stroke()
            }
            lastPoint = [posX, yPos]
        }
    }, [freqs, canvasRef.current])

    return <canvas ref={canvasRef} height={400} width={800}></canvas>
}
