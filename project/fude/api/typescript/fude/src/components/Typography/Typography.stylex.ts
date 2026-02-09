import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const headingLevels = {
    1: {
        size: "5xl",
    },
    2: {
        size: "4xl",
    },
    3: {
        size: "3xl",
    },
    4: {
        size: "2xl",
    },
    5: {
        size: "xl",
    },
    6: {
        size: "lg",
    },
} as const

export const typographyStyles = makeStyles({
    slots: create({
        text: {},
    }),
    conditions: {
        b: {},
        i: {},
    },
    variants: {
        color: {
            default: create({
                text: {
                    color: colors.foreground,
                },
            }),
            secondary: create({
                text: {
                    color: colors.secondaryForeground,
                },
            }),
            primary: create({
                text: {
                    color: colors.primary,
                },
            }),
        },
        size: {
            xs: create({
                text: {
                    fontSize: fontSize.xs,
                },
            }),
            sm: create({
                text: {
                    fontSize: fontSize.sm,
                },
            }),
            md: create({
                text: {
                    fontSize: fontSize.md,
                },
            }),
            lg: create({
                text: {
                    fontSize: fontSize.lg,
                },
            }),
            xl: create({
                text: {
                    fontSize: fontSize.xl,
                },
            }),
            "2xl": create({ text: { fontSize: fontSize["2xl"] } }),
            "3xl": create({ text: { fontSize: fontSize["3xl"] } }),
            "4xl": create({ text: { fontSize: fontSize["4xl"] } }),
            "5xl": create({ text: { fontSize: fontSize["5xl"] } }),
            "6xl": create({ text: { fontSize: fontSize["6xl"] } }),
            "7xl": create({ text: { fontSize: fontSize["7xl"] } }),
            "8xl": create({ text: { fontSize: fontSize["8xl"] } }),
        },
    },
    defaultVariants: {
        color: "default",
        size: "md",
    },
})
