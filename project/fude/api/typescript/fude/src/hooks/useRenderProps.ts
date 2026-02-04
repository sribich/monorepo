import { type ReactNode, useMemo } from "react"

import type { RenderProps } from "../utils/props"

export const useRenderProps = <T>(props: RenderProps<T>, values: T) => {
    const { children, className, style } = props

    /**
     * TODO: I'm pretty sure values changes every render here. Can we verify?
     */
    return useMemo(() => {
        return {
            children: typeof children === "function" ? children(values) : (children as ReactNode),
            className: typeof className === "function" ? className(values) : className,
            style: typeof style === "function" ? style(values) : style,
        }
    }, [children, className, style, values])
}

export const useStyleProps = <T = undefined>(
    props: Omit<RenderProps<T>, "children">,
    values: T,
) => {
    const { className, style } = props

    /**
     * TODO: I'm pretty sure values changes every render here. Can we verify?
     */
    return useMemo(() => {
        return {
            className: typeof className === "function" ? className(values) : className,
            style: typeof style === "function" ? style(values) : style,
        }
    }, [className, style, values])
}
