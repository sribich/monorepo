import { makeStyles } from "@sribich/fude"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { create } from "@stylexjs/stylex"

export const unstyledListItemStyles = makeStyles({
    slots: create({
        container: {
            fontSize: fontSize.lg,
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
