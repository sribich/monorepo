import { makeStyles } from "@sribich/fude"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { create } from "@stylexjs/stylex"

export const sectionNameStyles = makeStyles({
    slots: create({
        container: {
            color: colors.foreground,
            marginTop: 56,
            marginBottom: 16,
            fontSize: "26px",
            fontWeight: 500,
            "::after": {
                content: "",
                display: "block",
                backgroundColor: "#3d3d3d",
                height: "2px",
                width: "100%",
            },
        },
        content: {},
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
