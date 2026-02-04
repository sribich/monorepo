import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const linkStyles = makeStyles({
    slots: create({
        link: {
            textDecoration: "none",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
