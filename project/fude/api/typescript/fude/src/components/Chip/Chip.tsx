import { mergeProps } from "react-aria"

import { useRenderProps } from "../../hooks/useRenderProps"
import { stylexColorVariantsNonInteractive } from "../../theme/atomics/color"
import { type VariantProps, useStyles } from "../../theme/props"
import type { RenderProps } from "../../utils/props"
import { chipStyles } from "./Chip.styles"
import type { RefObject } from "react"

////////////////////////////////////////////////////////////////////////////////
/// Chip
////////////////////////////////////////////////////////////////////////////////
export interface ChipProps extends RenderProps<undefined>, VariantProps<typeof chipStyles> {
    ref?: RefObject<HTMLDivElement>
    /**
     * A custom color for the chip
     */
    rawColor?: string | undefined
}

export const Chip = (props: ChipProps) => {
    const {
        styles,
        values: { variant, color },
    } = useStyles(chipStyles, props)

    const renderProps = useRenderProps(props, undefined)
    const styleProps = styles.chipContainer(
        stylexColorVariantsNonInteractive[variant][color],
        props.rawColor && styles.chipContainer.customColor(props.rawColor),
    )

    return (
        <div {...mergeProps(props, styleProps, renderProps)} ref={props.ref}>
            {renderProps.children}
        </div>
    )
}
