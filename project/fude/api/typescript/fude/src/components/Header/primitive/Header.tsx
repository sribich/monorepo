import type { RefObject, HTMLAttributes } from "react"

import { createControlledContext } from "../../../hooks/context"

////////////////////////////////////////////////////////////////////////////////
/// Utils
////////////////////////////////////////////////////////////////////////////////
export const [useHeaderContext, HeaderContext] = createControlledContext<
    HTMLAttributes<HTMLElement>,
    HTMLElement
>()

////////////////////////////////////////////////////////////////////////////////
/// Header
////////////////////////////////////////////////////////////////////////////////
export interface HeaderProps extends HTMLAttributes<HTMLElement> {
    ref?: RefObject<HTMLElement>
}

export const Header = (_props: HeaderProps) => {
    const [props, ref] = useHeaderContext(_props)

    // // Do we need to pass the original shit through?
    // const collectionNode = useShallowRender("header", props, ref)
    //
    //     if (collectionNode) {
    //         return collectionNode
    //     }

    return <header {...props} ref={ref} />
}
