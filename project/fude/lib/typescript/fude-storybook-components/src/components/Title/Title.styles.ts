import { makeStyles } from "@sribich/fude"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { create } from "@stylexjs/stylex"

export const titleStyles = makeStyles({
    slots: create({
        container: {
            color: colors.foreground,
            marginBottom: 12,
            fontSize: "24px",
            fontWeight: 300,
        },
        content: {},
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
