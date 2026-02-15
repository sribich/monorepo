import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { fonts } from "@sribich/fude-theme/vars/fonts.stylex"
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
        base: {},
        heading: {
            fontFamily: fonts.display,
        },
        text: {
            fontFamily: fonts.default,
        },
    }),
    conditions: {
        b: {},
        i: {},
    },
    variants: {
        color: {
            default: create({
                base: {
                    color: colors.foreground,
                },
            }),
            secondary: create({
                base: {
                    color: colors.secondaryForeground,
                },
            }),
            primary: create({
                base: {
                    color: colors.primary,
                },
            }),
        },
        size: {
            xs: create({
                base: {
                    fontSize: fontSize.xs,
                },
            }),
            sm: create({
                base: {
                    fontSize: fontSize.sm,
                },
            }),
            md: create({
                base: {
                    fontSize: fontSize.md,
                },
            }),
            lg: create({
                base: {
                    fontSize: fontSize.lg,
                },
            }),
            xl: create({
                base: {
                    fontSize: fontSize.xl,
                },
            }),
            "2xl": create({ base: { fontSize: fontSize["2xl"] } }),
            "3xl": create({ base: { fontSize: fontSize["3xl"] } }),
            "4xl": create({ base: { fontSize: fontSize["4xl"] } }),
            "5xl": create({ base: { fontSize: fontSize["5xl"] } }),
            "6xl": create({ base: { fontSize: fontSize["6xl"] } }),
            "7xl": create({ base: { fontSize: fontSize["7xl"] } }),
            "8xl": create({ base: { fontSize: fontSize["8xl"] } }),
        },
    },
    defaultVariants: {
        color: "default",
        size: "md",
    },
})
