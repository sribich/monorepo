import type { LabelHTMLAttributes, RefObject } from "react"

import { createControlledContext } from "../../hooks/context"

////////////////////////////////////////////////////////////////////////////////
/// Label
////////////////////////////////////////////////////////////////////////////////
export const [useLabelContext, LabelContext] = createControlledContext<
    LabelProps,
    HTMLLabelElement
>()

export interface LabelProps extends LabelHTMLAttributes<HTMLLabelElement> {
    ref?: RefObject<HTMLLabelElement>
}

export const Label = (_props: LabelProps) => {
    const [props, ref] = useLabelContext(_props)

    return <label {...props} ref={ref} />
}
