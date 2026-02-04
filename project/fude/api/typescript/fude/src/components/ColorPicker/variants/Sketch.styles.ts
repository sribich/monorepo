import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const sketchStyles = makeStyles({
    slots: create({
        container: {
            zIndex: 1,
            position: "relative",
            padding: spacing["2"],
            borderRadius: borderRadius.md,
            boxShadow: boxShadow.md,
            width: spacing["72"],
        },
        area: {
            zIndex: 2,
            height: spacing["56"],
            width: "100%",
        },

        controlGroup: {
            display: "flex",
            flexDirection: "row",
            gap: spacing["2"],
        },
        sliderGroup: {
            zIndex: 2,
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            flex: "1 1 0%",
        },
        colorSquare: {
            zIndex: 1,
            height: spacing["8"],
            width: spacing["8"],
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
