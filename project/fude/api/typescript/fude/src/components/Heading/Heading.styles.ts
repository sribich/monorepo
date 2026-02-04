import { create } from "@stylexjs/stylex"
import { makeStyles } from "../../theme/props"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"

export const headingStyles = makeStyles({
    slots: create({
        container: {},
    }),
    conditions: {},
    variants: {
        size: {
            md: {
                fontSize: fontSize["lg"],
            },
        },
    },
    defaultVariants: {
        size: "md",
    },
})
