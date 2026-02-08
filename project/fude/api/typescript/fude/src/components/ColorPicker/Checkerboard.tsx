import type { RefObject } from "react"

import { useStyleProps } from "../../hooks/useRenderProps"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import type { StyleProps } from "../../utils/props"
import { checkerboardStyles } from "./Checkerboard.stylex"
import { checkerboard } from "./util"

//==============================================================================
// Checkerboard
//==============================================================================
export namespace Checkerboard {
    export interface Props extends StyleProps, VariantProps<typeof checkerboardStyles> {
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
}

export const Checkerboard = (props: Checkerboard.Props) => {
    const { colorA = "#00000000", colorB = "#00000020", cellSize = 8 } = props

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
