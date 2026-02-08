import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { getReadableColor } from "../../theme/colors"
import { makeStyles } from "../../theme/props"

export const chipStyles = makeStyles({
    slots: create({
        chipContainer: {
            display: "inline-flex",
            boxSizing: "border-box",
            position: "relative",
            width: "fit-content",
            alignItems: "center",
            justifyContent: "space-between",
            whiteSpace: "nowrap",
        },
        chipContent: {},
    }),
    modifiers: {
        customColor: {
            chipContainer: (color: string) => ({
                color: getReadableColor(color),
                backgroundColor: color,
            }),
        },
    },
    variants: {
        color: {
            default: create({}),
            primary: create({}),
            secondary: create({}),
            success: create({}),
            warning: create({}),
            danger: create({}),
        },
        radius: {
            sm: create({
                chipContainer: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                chipContainer: {
                    borderRadius: borderRadius.md,
                },
            }),
            full: create({
                chipContainer: {
                    borderRadius: borderRadius.full,
                },
            }),
        },
        size: {
            sm: create({
                chipContainer: {
                    height: spacing["6"],
                    paddingLeft: spacing["1"],
                    paddingRight: spacing["1"],
                    fontSize: fontSize.xs,
                },
                chipContent: {},
            }),
            md: create({
                chipContainer: {
                    height: spacing["6"],
                    paddingLeft: spacing["2"],
                    paddingRight: spacing["2"],
                    paddingTop: spacing["1"],
                    paddingBottom: spacing["1"],
                    fontSize: fontSize.sm,
                },
                chipContent: {},
            }),
            lg: create({
                chipContainer: {
                    height: spacing["8"],
                    paddingLeft: spacing["2"],
                    paddingRight: spacing["2"],
                    fontSize: fontSize.md,
                },
                chipContent: {},
            }),
        },
        variant: {
            solid: create({}),
        },
    },
    defaultVariants: {
        color: "default",
        radius: "full",
        size: "md",
        variant: "solid",
    },
})
