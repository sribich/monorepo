import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { absoluteComponentSize } from "@sribich/fude-theme/vars/sizing.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"

export const listBoxStyles = makeStyles({
    slots: create({
        wrapper: {
            width: "100%",
            position: "relative",
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["4"],
            padding: newSpacing["4"],
            overflow: "clip",
        },
        list: {
            width: "100%",
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["2"],
            outlineWidth: "1px",
            outlineStyle: "solid",
            outlineColor: "transparent",
        },

        container: {
            backgroundColor: colors.background,
            display: "flex",
            flexDirection: "column",
            width: "100%",
            padding: spacing["1"],
            gap: spacing["0.5"],

            boxShadow: boxShadow.sm,
            boxSizing: "border-box",
        },
        itemWrapper: {
            display: "flex",
            gap: newSpacing["8"],
            alignItems: "center",
            justifyContent: "space-between",
            position: "relative",
            paddingBlock: newSpacing["8"],
            paddingInline: newSpacing["6"],
            width: "100%",
            height: "100%",
            cursor: "pointer",

            backgroundColor: colors.background,
            // ...include(padding["x-1"]),
            // ...include(padding["y-0.5"]),
        },
        dropIndicator: {
            outlineWidth: "1px",
            outlineStyle: "solid",
            outlineColor: colors.primary,
        },
    }),
    variants: {
        rounded: {
            none: create({
                wrapper: {
                    borderRadius: borderRadius.none,
                },
                itemWrapper: {
                    borderRadius: borderRadius.none,
                },
            }),
            sm: create({
                wrapper: {
                    borderRadius: borderRadius.sm,
                },
                itemWrapper: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                wrapper: {
                    borderRadius: borderRadius.sm,
                },
                itemWrapper: {
                    borderRadius: borderRadius.sm,
                },
            }),
            lg: create({
                wrapper: {
                    borderRadius: borderRadius.lg,
                },
                itemWrapper: {
                    borderRadius: borderRadius.lg,
                },
            }),
        },
        size: {
            sm: create({
                wrapper: {},
                itemWrapper: {
                    height: absoluteComponentSize.sm,
                },
            }),
            md: create({
                wrapper: {},
                itemWrapper: {
                    height: absoluteComponentSize.md,
                },
            }),
            lg: create({
                wrapper: {},
                itemWrapper: {
                    height: absoluteComponentSize.lg,
                },
            }),
        },
        highlightChildren: {
            true: create({
                itemWrapper: {
                    ":hover": {
                        color: colors.backgroundHoverForeground,
                        backgroundColor: colors.backgroundHover,
                    },
                },
            }),
            false: create({}),
        },
    },
    defaultVariants: {
        highlightChildren: true,
        rounded: "md",
        size: "md",
    },
    modifiers: {},
    conditions: {},
    // compounds: {},
})

// export const listBoxItemStyles = makeStyles({})

export const ListBoxStyles = createNewGenericContext<CachedStyles<typeof listBoxStyles>>()
