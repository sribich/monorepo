import { useStyles } from "@sribich/fude"
import { Check, X } from "lucide-react"
import type { ReactNode } from "react"

import { componentRuleStyles } from "./ComponentRule.styles"

export interface ComponentRuleProps {
    component: ReactNode
    description: ReactNode
    isRecommended?: boolean
}

export const ComponentRule = (props: ComponentRuleProps) => {
    const { styles } = useStyles(componentRuleStyles, {})

    const titleIcon = props.isRecommended ? (
        <Check {...styles.icon(styles.icon.isRecommended)} />
    ) : (
        <X {...styles.icon()} />
    )
    const title = props.isRecommended ? "Do" : "Don't"

    return (
        <section>
            <figure {...styles.container()}>{props.component}</figure>
            <h5 {...styles.title()}>
                {titleIcon}
                {title}
            </h5>
            <section {...styles.description()}>{props.description}</section>
        </section>
    )
}
