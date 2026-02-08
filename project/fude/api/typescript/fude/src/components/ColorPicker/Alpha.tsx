import type { Color } from "@react-types/color"
import { useMemo } from "react"

import { useStyles } from "../../theme/props"
import { Slider, SliderTrack } from "../Slider/Slider"
import { alphaStyles } from "./Alpha.styles"
import { Checkerboard } from "./Checkerboard"

//==============================================================================
// Alpha
//==============================================================================
export namespace Alpha {
    export interface Props {
        /**
         * The current target color to change the alpha of.
         */
        color: Color
        /**
         * A handler called every time the slider value changes.
         */
        onChange?: (color: Color) => void
        /**
         * A handler called when the slider stops moving due to being let go.
         */
        onCommit?: (color: Color) => void
    }
}

export const Alpha = (props: Alpha.Props) => {
    const rgbaColor = props.color.toFormat("rgba")
    const lower = rgbaColor.withChannelValue("alpha", 0)
    const upper = rgbaColor.withChannelValue("alpha", 1)

    const gradient = useMemo(() => {
        return `linear-gradient(to right, ${lower.toString("css")}, ${upper.toString("css")})`
    }, [lower, upper])

    const onChange = (value: number) => {
        props.onChange?.(rgbaColor.withChannelValue("alpha", value / 100))
    }

    const onCommit = (value: number) => {
        props.onCommit?.(rgbaColor.withChannelValue("alpha", value / 100))
    }

    return (
        <Slider
            value={rgbaColor.getChannelValue("alpha") * 100}
            size="md"
            onChange={onChange}
            onChangeEnd={onCommit}
            trackGradient={gradient}
            renderTrack={AlphaTrack}
        />
    )
}

//==============================================================================
// AlphaTrack
//==============================================================================
export namespace AlphaTrack {
    export interface Props extends SliderTrack.Props {}
}

const AlphaTrack = (props: AlphaTrack.Props) => {
    const { styles } = useStyles(alphaStyles, props)

    return (
        <SliderTrack {...props}>
            <div {...styles.alpha()}>
                <Checkerboard />
            </div>
            {props.children}
        </SliderTrack>
    )
}
