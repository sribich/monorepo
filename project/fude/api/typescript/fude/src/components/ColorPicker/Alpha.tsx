import type { Color } from "@react-types/color"

import { useStyles } from "../../theme/props"
import { Slider, SliderThumb, SliderTrack } from "../Slider/Slider"
import { alphaStyles } from "./Alpha.styles"
import { Checkerboard } from "./Checkerboard"

////////////////////////////////////////////////////////////////////////////////
/// Alpha
////////////////////////////////////////////////////////////////////////////////
export interface AlphaProps {
    /**
     * The current target color to change the alpha of.
     */
    color: Color
    /**
     * A handler called every time the slider value changes.
     */
    onChange?(color: Color): void
    /**
     * A handler called when the slider stops moving due to
     * being let go.
     */
    onCommit?(color: Color): void
}

export const Alpha = (props: AlphaProps) => {
    const rgbaColor = props.color.toFormat("rgba")
    const lower = rgbaColor.withChannelValue("alpha", 0)
    const upper = rgbaColor.withChannelValue("alpha", 1)

    const onChange = (value: number) => {
        props.onChange?.(rgbaColor.withChannelValue("alpha", value / 100))
    }
    const onCommit = (value: number) => {
        props.onChange?.(rgbaColor.withChannelValue("alpha", value / 100))
    }

    const { styles } = useStyles(alphaStyles, props)

    return (
        <Slider
            value={rgbaColor.getChannelValue("alpha") * 100}
            label=""
            size="md"
            onChange={onChange}
            onChangeEnd={onCommit}
        >
            <SliderTrack>
                <div {...styles.alphaContainer()}>
                    <Checkerboard />
                    <div
                        {...styles.alphaSlider()}
                        style={{
                            background: `linear-gradient(to right, ${lower.toString(
                                "css",
                            )}, ${upper.toString("css")})`,
                        }}
                    />
                </div>
            </SliderTrack>
            <SliderThumb slot="lower" />
        </Slider>
    )
}

/* import { useState } from "react"

import { Slider } from "../../slider/primitive/Slider"
import { HslColor } from "../color"
import { Checkerboard } from "./Checkerboard"

export type AlphaProps = {
    color: HslColor["value"]
    setColor: (color: HslColor["value"]) => void
}

export const Alpha = ({ color, setColor }: AlphaProps) => {
    const alpha = (color.a || 1) * 100

    const onChange = (values: number | number[]) => {
        const value = Array.isArray(values) ? values?.[0] ?? 0 : values ?? 0

        setColor({ ...color, a: value / 100 })
    }

    // TODO: Implement this
    const onCommit = (value) => {}

    const trackStyles = {
        background: `linear-gradient(to right, hsl(${color.h * 360}, ${color.hsl.s * 100}%, ${
            color.hsl.l * 100
        }%, 0), hsl(${color.h * 360}, ${color.hsl.s * 100}%, ${color.hsl.l * 100}%, 1))`,
    }

    return (
        <Slider value={alpha} maxValue={100} onChange={onChange} onChangeEnd={onCommit}>
            <Slider.Track
                className="relative w-full h-1.5"
                renderThumb={({ index, state }) => (
                    <Slider.Thumb
                        key={index}
                        index={index}
                        className="w-3 h-3 bg-transparent border-white rounded-full shadow border-[3px] outline-1 outline-black/40"
                    />
                )}
            >
                <Checkerboard className="absolute inset-0 h-full rounded shadow-inner top-[50%] -translate-y-[50%]" />
                <div
                    style={trackStyles}
                    className="absolute inset-0 h-full rounded shadow-inner top-[50%] -translate-y-[50%]"
                />
            </Slider.Track>
        </Slider>
    )
}
 */
