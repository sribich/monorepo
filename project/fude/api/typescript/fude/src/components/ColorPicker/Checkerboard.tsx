import type { RefObject } from "react"
import { useStyleProps } from "../../hooks/useRenderProps"
import { type VariantProps, useStyles } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import type { StyleProps } from "../../utils/props"
import { checkerboardStyles } from "./Checkerboard.styles"
import { checkerboard } from "./util"

////////////////////////////////////////////////////////////////////////////////
/// Checkerboard
////////////////////////////////////////////////////////////////////////////////
export interface CheckerboardProps extends StyleProps, VariantProps<typeof checkerboardStyles> {
    ref?: RefObject<HTMLDivElement>

    /**
     * The default primary color of the checkerboard
     * @default #00000000
     */
    colorA?: string
    /**
     * The secondary color of the checkerboard
     * @default #00000020
     */
    colorB?: string
    /**
     * The size of an individual checkerboard square, in px
     * @default 8
     */
    cellSize?: number
}

export const Checkerboard = (props: CheckerboardProps) => {
    const { colorA = "#00000000", colorB = "#00000020", cellSize = 8 } = props

    /*
    00000000
    00000020

    colorA = "#eaeaea",
    colorB = "#949494",

    565656
    282828


    1b1b1b
    3b3b3b
    */

    const styleProps = useStyleProps(props, {})

    const { styles } = useStyles(checkerboardStyles, props)

    return (
        <div
            {...mergeProps(
                styles.checkerboard(
                    styles.checkerboard.background(
                        `url(${checkerboard(colorA, colorB, cellSize)}) center left`,
                    ),
                ),
                styleProps,
            )}
            ref={props.ref}
        />
    )
}
