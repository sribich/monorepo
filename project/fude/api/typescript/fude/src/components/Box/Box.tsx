import type { ReactNode } from "react"

import { createNewControlledContext, type SlotProps } from "../../hooks/context"
import type { StyleProps } from "../../utils/props"

//==============================================================================
// Box Utils
//==============================================================================
export const BoxContext = createNewControlledContext<Box.Props, HTMLDivElement>()

//==============================================================================
// Box
//==============================================================================
export namespace Box {
    export interface Props extends SlotProps, StyleProps {
        children?: ReactNode
    }
}

export const Box = (_props: Box.Props) => {
    const [props, ref] = BoxContext.useContext(_props)

    return <div {...props} ref={ref} />
}
