import type { ElementType, ReactNode, RefObject } from "react"
import { createControlledContext, type SlotProps } from "../../hooks/context"

export interface HeadingProps extends SlotProps {
    ref?: RefObject<HTMLHeadingElement>
    /**
     * The content of the heading.
     */
    children: ReactNode

    level?: 1 | 2 | 3 | 4 | 5 | 6
}

const [useHeadingContext, HeadingProvider] = createControlledContext<
    HeadingProps,
    HTMLHeadingElement
>()
export { HeadingProvider }

export const HeadingOld = (_props: HeadingProps) => {
    const [props, ref] = useHeadingContext(_props)

    const HeadingTag = `h${props.level ?? 2}` as ElementType

    /*
    // const [props, forwardedRef] = useHeaderContext(originalProps, originalForwardedRef)

    const collectionNode = useShallowRender("header", originalProps, originalForwardedRef)

    if (collectionNode) {
        return collectionNode
    }

    return <header {...props} ref={forwardedRef} />
    */

    return (
        <HeadingTag {...props} ref={ref}>
            {props.children}
        </HeadingTag>
    )
}
