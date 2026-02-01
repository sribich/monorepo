import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { makeStyles } from "../../theme/props"

export const flexStyles = makeStyles({
    slots: create({
        flex: {
            display: "flex",
        },
    }),
    conditions: {
        inline: create({
            flex: { display: "inline-flex" },
        }),
    },
    variants: {
        direction: {
            row: create({
                flex: { flexDirection: "row" },
            }),
            column: create({
                flex: { flexDirection: "column" },
            }),
            "row-reverse": create({
                flex: { flexDirection: "row-reverse" },
            }),
            "column-reverse": create({
                flex: { flexDirection: "column-reverse" },
            }),
        },
        gap: {
            sm: create({
                flex: { gap: spacing["1.5"] },
            }),
            md: create({
                flex: { gap: spacing["3"] },
            }),
            lg: create({
                flex: { gap: spacing["6"] },
            }),
        },
        items: {
            "flex-start": create({
                flex: { alignItems: "flex-start" },
            }),
            "flex-end": create({
                flex: { alignItems: "flex-end" },
            }),
            center: create({
                flex: { alignItems: "center" },
            }),
            stretch: create({
                flex: { alignItems: "stretch" },
            }),
            baseline: create({
                flex: { alignItems: "baseline" },
            }),
        },
        justify: {
            "flex-start": create({
                flex: { justifyContent: "flex-start" },
            }),
            "flex-end": create({
                flex: { justifyContent: "flex-end" },
            }),
            center: create({
                flex: { justifyContent: "center" },
            }),
            "space-between": create({
                flex: { justifyContent: "space-between" },
            }),
            "space-around": create({
                flex: { justifyContent: "space-around" },
            }),
            "space-evenly": create({
                flex: { justifyContent: "space-evenly" },
            }),
        },
        wrap: {
            nowrap: create({
                flex: { flexWrap: "nowrap" },
            }),
            wrap: create({
                flex: { flexWrap: "wrap" },
            }),
            "wrap-reverse": create({
                flex: { flexWrap: "wrap-reverse" },
            }),
        },
    },
    defaultVariants: {
        direction: "row",
        gap: "sm",
        items: "stretch",
        justify: "flex-start",
        wrap: "nowrap",
    },
})
