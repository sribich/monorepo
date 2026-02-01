import type { ReactNode, Ref } from "react"

import { useStyleProps } from "../../hooks/useRenderProps"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import type { StyleProps } from "../../utils/props"
import { flexStyles } from "./Flex.stylex"

//==============================================================================
// Flex
//==============================================================================
export namespace Flex {
    export interface Props extends StyleProps, VariantProps<typeof flexStyles> {
        ref?: Ref<HTMLDivElement>
        children: ReactNode
    }
}

export const Flex = (props: Flex.Props) => {
    const { styles } = useStyles(flexStyles, props)
    const styleProps = useStyleProps(props, {})

    return (
        <div {...mergeProps(styles.flex(), styleProps)} ref={props.ref}>
            {props.children}
        </div>
    )
}
