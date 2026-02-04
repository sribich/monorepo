import { useStyles } from "@sribich/fude"
import type { ReactNode } from "react"

import { ComponentRule } from "../ComponentRule/ComponentRule"
import { componentRulesStyles } from "./ComponentRules.styles"

export interface ComponentRulesProps {
    rules: {
        positive: {
            component: ReactNode
            description: ReactNode
        }
        negative: {
            component: ReactNode
            description: ReactNode
        }
    }[]
}

export const ComponentRules = (props: ComponentRulesProps) => {
    const { styles } = useStyles(componentRulesStyles, {})

    return (
        <article {...styles.container()}>
            {props.rules.map((rule, index) => (
                <section {...styles.rulePair()} key={index}>
                    <ComponentRule
                        component={rule.positive.component}
                        description={rule.positive.description}
                        isRecommended
                    />
                    <ComponentRule
                        component={rule.negative.component}
                        description={rule.negative.description}
                    />
                </section>
            ))}
        </article>
    )
}
