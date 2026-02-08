import type { Color } from "@react-types/color"

import { Slider } from "../Slider/Slider"

/**
 * The stops in a linear-gradient css function required to
 * display the full hue spectrum.
 */
const hueGradientStops = [
    "rgb(255, 0, 0)",
    "rgb(255, 255, 0)",
    "rgb(0, 255, 0)",
    "rgb(0, 255, 255)",
    "rgb(0, 0, 255)",
    "rgb(255, 0, 255)",
    "rgb(255, 0, 0)",
].join(", ")

const gradient = `linear-gradient(to right, ${hueGradientStops})`

//==============================================================================
// Hue
//==============================================================================
export namespace Hue {
    export interface Props {
        color: Color

        onChange?: (color: Color) => void
        onCommit?: (color: Color) => void
    }
}

export const Hue = (props: Hue.Props) => {
    const hsl = props.color.toFormat("hsl")

    const onChange = (value: number) => {
        props.onChange?.(hsl.withChannelValue("hue", value))
    }

    const onCommit = (value: number) => {
        props.onCommit?.(hsl.withChannelValue("hue", value))
    }

    return (
        <Slider
            minValue={0}
            maxValue={360}
            value={hsl.getChannelValue("hue")}
            onChange={onChange}
            onChangeEnd={onCommit}
            size="md"
            trackGradient={gradient}
        />
    )
}
