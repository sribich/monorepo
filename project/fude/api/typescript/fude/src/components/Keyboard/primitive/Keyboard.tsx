import type { HTMLAttributes, RefObject } from "react"

import { createControlledContext } from "../../../hooks/context"

////////////////////////////////////////////////////////////////////////////////
/// Utils
////////////////////////////////////////////////////////////////////////////////
export const [useKeyboardContext, KeyboardContext] = createControlledContext<
    HTMLAttributes<HTMLElement>,
    HTMLElement
>()

////////////////////////////////////////////////////////////////////////////////
/// Keyboard
////////////////////////////////////////////////////////////////////////////////
export interface KeyboardProps extends HTMLAttributes<HTMLElement> {
    ref?: RefObject<HTMLElement>
}

export const Keyboard = (_props: KeyboardProps) => {
    const [props, ref] = useKeyboardContext(_props)

    return <kbd dir="ltr" {...props} ref={ref} />
}
