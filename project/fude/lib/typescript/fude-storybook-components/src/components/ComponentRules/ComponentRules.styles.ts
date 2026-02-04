import { makeStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"

export const componentRulesStyles = makeStyles({
    slots: create({
        container: {},
        content: {},
        rulePair: {
            display: "grid",
            gridTemplateColumns: "1fr 1fr",
            gap: "16px",
            marginBottom: "32px",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
