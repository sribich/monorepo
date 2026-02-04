import type {
    ComponentPropsWithRef,
    ComponentType,
    ElementType,
    HTMLAttributes,
    RefObject,
} from "react"

import { createControlledContext } from "../../../hooks/context"

////////////////////////////////////////////////////////////////////////////////
/// Utils
////////////////////////////////////////////////////////////////////////////////
export const [useTextContext, TextContext] = createControlledContext<TextProps, HTMLElement>()

////////////////////////////////////////////////////////////////////////////////
/// Text
////////////////////////////////////////////////////////////////////////////////
export interface TextProps extends HTMLAttributes<HTMLElement> {
    ref?: RefObject<HTMLElement>

    elementType?: string
}

export const Text = (_props: TextProps) => {
    const [props, ref] = useTextContext(_props)

    const Component = (props.elementType ?? "span") as unknown as ComponentType<
        ComponentPropsWithRef<ElementType>
    >

    return <Component {...props} ref={ref} />
}
