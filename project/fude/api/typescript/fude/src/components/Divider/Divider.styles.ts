import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const dividerStyles = makeStyles({
    slots: create({
        divider: {
            backgroundColor: colors.borderLayout,
            writingMode: "horizontal-tb",
        },
    }),
    conditions: {},
    variants: {
        size: {
            sm: create({
                divider: {
                    "--divider-size": borderWidth.sm,
                    // margin: `${spacing["1"]} 0`,
                },
            }),
            md: create({
                divider: {
                    "--divider-size": borderWidth.md,
                    // margin: `${spacing["1.5"]} 0`,
                },
            }),
            lg: create({
                divider: {
                    "--divider-size": borderWidth.lg,
                    // margin: `${spacing["2"]} 0`,
                },
            }),
        },
        orientation: {
            vertical: create({
                divider: {
                    blockSize: "auto",
                    alignSelf: "stretch",
                    inlineSize: "var(--divider-size)",
                },
            }),
            horizontal: create({
                divider: {
                    inlineSize: "100%",
                    blockSize: "var(--divider-size)",
                },
            }),
        },
    },
    defaultVariants: {
        size: "sm",
        orientation: "horizontal",
    },
})
