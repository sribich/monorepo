import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create, when } from "@stylexjs/stylex"
import { makeStyles } from "../../theme/props"

export const checkboxStyles = makeStyles({
    slots: create({
        base: {
            position: "relative",
            display: "inline-flex",
            maxWidth: "fit-content",
            cursor: "pointer",
            alignItems: "center",
            justifyContent: "start",
            padding: newSpacing["8"],
        },
        wrapper: {
            position: "relative",
            display: "inline-flex",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
            flexShrink: 0,
            [when.ancestor(":is([data-pressed])")]: {
                scale: 0.95,
            },
            [when.ancestor(":hover")]: {
                backgroundColor: colors.backgroundSecondary,
            },
            "::before": {
                content: "",
                position: "absolute",
                inset: 0,
                borderStyle: "solid",
                borderWidth: borderWidth.md,
                boxSizing: "border-box",
                borderColor: colors.borderUi,
            },
            "::after": {
                content: "",
                position: "absolute",
                inset: 0,
                transform: "scale(50%)",
                opacity: "0",
                transformOrigin: "center",
                transitionProperty: "transform, opacity",
                transitionDuration: "250ms",
                transitionTimingFunction: ".4, 0, .2, 1",
            },
        },
        icon: {
            zIndex: "10",
            height: newSpacing["12"],
            width: newSpacing["16"],
            opacity: 0,
        },
        label: {
            marginLeft: newSpacing["8"],
        },
    }),
    modifiers: {
        selected: create({
            icon: {
                opacity: "100",
            },
            wrapper: {
                "::after": {
                    transform: "scale(100%)",
                    opacity: "100",
                },
            },
        }),
    },
    variants: {
        color: {
            default: create({
                wrapper: {
                    "::after": {
                        backgroundColor: colors.background,
                    },
                },
            }),
            primary: create({
                wrapper: {
                    color: colors.primaryForeground,
                    "::after": {
                        color: colors.primaryForeground,
                        backgroundColor: colors.primary,
                    },
                },
            }),
            secondary: create({
                wrapper: {},
            }),
            success: create({
                wrapper: {},
            }),
            warning: create({
                wrapper: {},
            }),
            danger: create({
                wrapper: {},
            }),
        },
        size: {
            sm: create({
                wrapper: {
                    height: newSpacing["16"],
                    width: newSpacing["16"],
                },
            }),
            md: create({
                wrapper: {
                    height: newSpacing["20"],
                    width: newSpacing["20"],
                },
            }),
            lg: create({
                wrapper: {
                    height: newSpacing["24"],
                    width: newSpacing["24"],
                },
            }),
        },
        radius: {
            none: create({
                wrapper: {
                    borderRadius: {
                        default: borderRadius.none,
                        "::before": borderRadius.none,
                        "::after": borderRadius.none,
                    },
                },
            }),
            sm: create({
                wrapper: {
                    borderRadius: {
                        default: borderRadius.sm,
                        "::before": borderRadius.sm,
                        "::after": borderRadius.sm,
                    },
                },
            }),
            md: create({
                wrapper: {
                    borderRadius: {
                        default: borderRadius.sm,
                        "::before": borderRadius.sm,
                        "::after": borderRadius.sm,
                    },
                },
            }),
            lg: create({
                wrapper: {
                    borderRadius: {
                        default: borderRadius.sm,
                        "::before": borderRadius.sm,
                        "::after": borderRadius.sm,
                    },
                },
            }),
            full: create({
                wrapper: {
                    borderRadius: {
                        default: borderRadius.full,
                        "::before": borderRadius.full,
                        "::after": borderRadius.full,
                    },
                },
            }),
        },
    },
    defaultVariants: {
        color: "primary",
        size: "md",
        radius: "md",
    },
})
