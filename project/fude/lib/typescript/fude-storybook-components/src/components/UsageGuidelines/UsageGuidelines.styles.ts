import { makeStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"

export const usageGuidelinesStyles = makeStyles({
    slots: create({
        container: {},
        content: {
            margin: "8px 4px",
            display: "flex",
            alignItems: "center",
        },
        icon: {
            marginRight: 8,
            fontSize: "16px",
            display: "flex",
            alignSelf: "flex-start",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
