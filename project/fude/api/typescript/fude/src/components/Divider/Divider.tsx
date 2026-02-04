import { mergeProps } from "@react-aria/utils"
import type { RefObject } from "react"
import {
    Separator as AriaSeparator,
    type SeparatorProps as AriaSeparatorProps,
} from "react-aria-components"

import { createControlledContext } from "../../hooks/context"
import { useStyles, type VariantProps } from "../../theme/props"
import type { StyleProps } from "../../utils/props"
import { dividerStyles } from "./Divider.styles"

//==============================================================================
// Divider Utils
//==============================================================================
export const [useDividerContext, DividerContext] = createControlledContext<
    Divider.Props,
    HTMLHRElement
>()

//==============================================================================
// Divider
//==============================================================================
export namespace Divider {
    export interface Props
        extends Omit<AriaSeparatorProps, "orientation">,
            StyleProps,
            VariantProps<typeof dividerStyles> {
        ref?: RefObject<HTMLHRElement>
    }
}

export const Divider = (_props: Divider.Props) => {
    const [props, ref] = useDividerContext(_props)
    const { styles } = useStyles(dividerStyles, props)

    return <AriaSeparator {...mergeProps(props, styles.divider())} ref={ref} />
}
