import type { CSSProperties, ReactNode } from "react"

export type RenderChild<T, TNodeType = ReactNode> = T extends never
    ? TNodeType
    : TNodeType | ((props: T) => TNodeType)

export interface StyleProps {
    className?: string | undefined
    style?: CSSProperties | undefined
}

export type StyleSlot = { className?: string; class?: string }

export interface NamedStyleSlots<T extends string> {
    styleSlots?: {
        [K in T]?: StyleSlot | undefined
    }
}

export interface StyleRenderProps<T> {
    className?: string | ((renderProps: T) => string) | undefined
    style?: CSSProperties | ((renderProps: T) => CSSProperties) | undefined
}

export interface RenderProps<T = never, TNodeType = ReactNode> extends StyleRenderProps<T> {
    children?: RenderChild<T, TNodeType> | undefined
}
