import type { RefObject } from "react"
import { type AriaLinkOptions, useLink } from "react-aria"
import { useObjectRef } from "../../hooks/useObjectRef"
import { useRenderProps } from "../../hooks/useRenderProps"
import { useStyles } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import type { RenderProps } from "../../utils/props"
import { Delegate } from "../Delegate"
import { linkStyles } from "./Link.styles"

////////////////////////////////////////////////////////////////////////////////
/// Link
////////////////////////////////////////////////////////////////////////////////
export interface LinkProps
    extends Omit<AriaLinkOptions, "elementType">,
        RenderProps<LinkRenderProps> {
    ref?: RefObject<HTMLAnchorElement>
}

export interface LinkRenderProps {
    isPressed: boolean
}

export const Link = (props: LinkProps) => {
    const ref = useObjectRef(props.ref)

    const { linkProps, isPressed } = useLink({ elementType: "a", ...props }, ref)
    const renderProps = useRenderProps(props, {
        isPressed,
    })

    const { styles } = useStyles(linkStyles, {})

    // TODO: target="_blank"
    // TODO: rel="noopener noreferrer"
    return (
        <a {...mergeProps(linkProps, renderProps, styles.link())} ref={ref}>
            {renderProps.children}
        </a>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// DelegateLink
////////////////////////////////////////////////////////////////////////////////
export const DelegateLink = (props: LinkProps) => {
    const ref = useObjectRef(props.ref)

    const { linkProps, isPressed } = useLink({ elementType: "a", ...props }, ref)
    const renderProps = useRenderProps(props, {
        isPressed,
    })

    return (
        <Delegate {...mergeProps(linkProps, renderProps)} ref={ref}>
            {renderProps.children}
        </Delegate>
    )
}
