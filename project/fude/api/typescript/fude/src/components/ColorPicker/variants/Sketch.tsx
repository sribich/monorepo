import { type Color, parseColor } from "@react-stately/color"
import { useMemo, useState } from "react"

import { useStyles } from "../../../theme/props"
import { Alpha } from "../Alpha"
import { ColorArea } from "../ColorArea"
import { ColorSquare } from "../ColorSquare"
import { Hue } from "../Hue"
import { sketchStyles } from "./Sketch.styles"

/* import { Color, Colors } from "../../color"
import { useColor } from "../../hook/useColor"
import { Alpha } from "../../primitive/Alpha"
import { Checkerboard } from "../../primitive/Checkerboard"
import { Hue } from "../../primitive/Hue"
import { Shade } from "../../primitive/Shade"
import { SketchFields } from "./SketchFields"

export type SketchProps<TColor extends Colors["value"]> = {
    
}


export const Sketch = <TColor extends Colors["value"]>(props: SketchProps<TColor>) => {
    const { color, setColor } = useColor(props.color ?? "#000000ff")

    return (
        <div className="p-2 rounded shadow">
            <div className="relative w-full h-64">
                <Shade color={color} setColor={setColor} />
            </div>

            <div className="flex flex-row py-2">
                <div className="flex flex-col justify-between flex-1 mr-2">
                    <Hue trackProps="h-1.5" color={color} setColor={setColor} />
                    <Alpha className="h-2 mt-2" color={color} setColor={setColor} />
                </div>
                <div className="relative w-6 h-6 overflow-hidden rounded">
                    <Checkerboard className="w-full h-full" />
                    <div
                        className="absolute top-0 bottom-0 left-0 right-0 rounded shadow-inner-sm"
                        style={{
                            backgroundColor: `hsl(${color.h * 360}, ${color.hsl.s * 100}%, ${color.hsl.l * 100}%, ${
                                color.a
                            })`,
                        }}
                    ></div>
                </div>
                <SketchFields />
            </div>
        </div>
    )
}
 */

interface SketchProps {
    color: string

    onChange?: (color: string) => void
    onCommit?: (color: string) => void
}

/**
 * https://www.sketch.com/docs/designing/styling/the-color-popover/
 */
export const Sketch = (props: SketchProps) => {
    const { originalColor, originalSpace } = useMemo(
        () =>
            ({
                originalColor: parseColor(props.color),
                originalSpace: props.color.startsWith("#")
                    ? props.color.length === 5 || props.color.length === 9
                        ? "hexa"
                        : "hex"
                    : undefined,
            }) as const,
        [props.color],
    )

    const [color, setColor] = useState(originalColor)

    const onChange = (color: Color) => {
        setColor(color)

        if (props.onChange) {
            props.onChange(color.toString(originalSpace ?? originalColor.getColorSpace()))
        }
    }
    const onCommit = (color: Color) => {
        if (props.onCommit) {
            props.onCommit(color.toString(originalSpace ?? originalColor.getColorSpace()))
        }
    }

    const { styles } = useStyles(sketchStyles, props)

    return (
        <div {...styles.container()}>
            <ColorArea
                {...styles.area()}
                rounded="md"
                value={color}
                onChange={onChange}
                onChangeEnd={onCommit}
            />
            <div {...styles.controlGroup()}>
                <div {...styles.sliderGroup()}>
                    <Hue color={color} onChange={onChange} onCommit={onCommit} />
                    <Alpha color={color} onChange={onChange} onCommit={onCommit} />
                </div>
                <div {...styles.colorSquare()}>
                    <ColorSquare color={color} />
                </div>
            </div>
        </div>
    )
}
