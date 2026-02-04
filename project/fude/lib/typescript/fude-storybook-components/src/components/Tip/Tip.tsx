import { type VariantProps, useStyles } from "@sribich/fude"
import type { ReactNode } from "react"

import { tipStyles } from "./Tip.styles"

export interface TipProps extends VariantProps<typeof tipStyles> {
    children: ReactNode

    title?: string
    emoji?: string
}

export const Tip = (props: TipProps) => {
    const { emoji = "", title = "Tip" } = props

    const { styles } = useStyles(tipStyles, {
        color: props.color ?? "default",
    })

    return (
        <div {...styles.container()}>
            <div {...styles.title()}>
                <span>{emoji}</span>
                <span>{title}</span>
            </div>
            <div {...styles.content()}>{props.children}</div>
        </div>
    )
}
