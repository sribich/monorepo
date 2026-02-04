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
]

////////////////////////////////////////////////////////////////////////////////
/// Hue
////////////////////////////////////////////////////////////////////////////////
export interface HueProps {
    color: Color

    onChange?: (color: Color) => void
    onCommit?: (color: Color) => void
}

export const Hue = (props: HueProps) => {
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
            label=""
            size="md"
            trackGradient={hueGradientStops}
        />
    )
}

/* import { cn } from "../../../util/utils"
import { Slider } from "../../slider/primitive/Slider"
import { HslColor } from "../color"

export type HueProps = {
    color: HslColor["value"]
    setColor: (color: HslColor["value"]) => void
    trackProps?: string
}

export const Hue = ({ color, setColor, trackProps }: HueProps) => {
    const onChange = (values: number | number[]) => {
        const value = Array.isArray(values) ? values?.[0] ?? 0 : values ?? 0

        setColor({ ...color, h: value / 360 })
    }

    const onCommit = (value) => {
        // console.log(hue, value)
    }

    return (
        <Slider value={color.h * 360} maxValue={360} onChange={onChange} onChangeEnd={onCommit}>
            <Slider.Track
                className="relative w-full h-1.5"
                renderThumb={({ index, state }) => (
                    <Slider.Thumb
                        key={index}
                        index={index}
                        className="w-3 h-3 bg-transparent border-white rounded-full shadow-md border-[3px] outline-1 outline-black/40"
                    />
                )}
            >
                <div className={cn("w-full rounded shadow-inner h-full bg-gradient-to-r-hue", trackProps)}></div>
            </Slider.Track>
        </Slider>
    )
}
 */
