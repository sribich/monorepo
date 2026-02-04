import { create } from "@stylexjs/stylex"

import { createGenericContext, createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { absoluteComponentSize } from "@sribich/fude-theme/vars/sizing.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const gridListStyles = makeStyles({
    slots: create({
        container: {
            // backgroundColor: colors.background,
            display: "flex",
            flexDirection: "column",
            width: "100%",
            padding: spacing["1"],
            gap: spacing["0.5"],
        },
        item: {
            display: "flex",
            alignItems: "center",

            // backgroundColor: colors.background,
            // ...include(padding["x-1"]),
            // ...include(padding["y-0.5"]),
        },
        dropIndicator: {
            outlineWidth: "1px",
            outlineStyle: "solid",
            outlineColor: colors.primary,
        },
    }),
    conditions: {},
    variants: {
        rounded: {
            none: create({
                container: {
                    borderRadius: borderRadius.none,
                },
                item: {
                    borderRadius: borderRadius.none,
                },
            }),
            sm: create({
                container: {
                    borderRadius: borderRadius.sm,
                },
                item: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                container: {
                    borderRadius: borderRadius.sm,
                },
                item: {
                    borderRadius: borderRadius.sm,
                },
            }),
            lg: create({
                container: {
                    borderRadius: borderRadius.lg,
                },
                item: {
                    borderRadius: borderRadius.lg,
                },
            }),
        },
        size: {
            sm: create({
                container: {},
                item: {
                    height: absoluteComponentSize.sm,
                },
            }),
            md: create({
                container: {},
                item: {
                    height: absoluteComponentSize.md,
                },
            }),
            lg: create({
                container: {},
                item: {
                    height: absoluteComponentSize.lg,
                },
            }),
        },
        highlightChildren: {
            true: create({
                item: {
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
        highlightChildren: false,
        rounded: "md",
        size: "md",
    },
})

export const GridListStyleProvider = createNewGenericContext<CachedStyles<typeof gridListStyles>>()
