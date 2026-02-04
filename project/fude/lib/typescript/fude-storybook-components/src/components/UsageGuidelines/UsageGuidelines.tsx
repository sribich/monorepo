import { useStyles } from "@sribich/fude"
import type { ReactNode } from "react"

import { usageGuidelinesStyles } from "./UsageGuidelines.styles"

export interface UsageGuidelinesProps {
    guidelines: ReactNode[]
}

export const UsageGuidelines = (props: UsageGuidelinesProps) => {
    const { styles } = useStyles(usageGuidelinesStyles, {})

    return (
        <article {...styles.container()}>
            {props.guidelines.map((guideline, index) => (
                <span key={String(index)} {...styles.content()}>
                    <span {...styles.icon()}>➡️</span>
                    <span>{guideline}</span>
                </span>
            ))}
        </article>
    )
}
