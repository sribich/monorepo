import type { ForwardedRef, ReactElement } from "react"

export interface DelegateNode extends ReactElement {
    ref?: ForwardedRef<HTMLElement>
}

export type Delegatable<ParentProps> =
    | ({ asChild?: false } & Omit<ParentProps, "asChild">)
    | ({ asChild: true; children: DelegateNode } & Omit<ParentProps, "asChild" | "children">)
