import { create } from "@stylexjs/stylex"

import { createGenericContext, createNewGenericContext } from "../../hooks/context"
import { padding } from "../../theme/atomics/padding"
import { type CachedStyles, type MadeStyles, makeStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const menuStyles = makeStyles({
    slots: create({
        menuWrapper: {
            width: spacing["56"],
            overflow: "auto",

            outline: "none",

            color: colors.foreground,
            ":hover": {
                color: colors.backgroundHoverForeground,
            },
            // "bg-white ring-1 ring-black ring-opacity-5 animate-in fade-in zoom-in-95 fill-mode-forwards origin-top-left",
        },
        sectionHeader: {
            // "text-foreground-500 pl-1 text-xs"
            paddingLeft: spacing["1"],
            fontSize: fontSize.xs,
        },
        itemWrapper: {
            position: "relative",
            marginBottom: spacing["0.5"],
            boxSizing: "border-box",
            display: "flex",
            width: "100%",
            cursor: "pointer",
            alignItems: "center",
            justifyContent: "space-between",
            outline: "none",
            borderRadius: borderRadius.sm,
            textDecoration: "none",
            color: colors.foreground,
            ":hover": {
                backgroundColor: colors.backgroundHover,
            },

            // item: " relative  items-center justify-betweentext-gray-900 subpixel-antialiased outline-none focus:bg-violet-500 focus:text-white",
        },
        itemContent: {
            display: "flex",
            width: "100%",
            flexDirection: "column",
            alignItems: "flex-start",
            justifyContent: "center",
        },
        itemLabel: {
            flex: "1 1 0%",
            fontSize: fontSize.sm,
            // label: "flex-1 truncate text-sm font-normal",
        },
        itemDescription: {
            // "text-foreground-500 group-hover:text-current"
            width: "100%",
            fontSize: fontSize.xs,
        },
        itemShortcut: {
            borderColor: colors.borderUi,
            borderWidth: borderWidth.sm,
            borderStyle: "solid",
            borderRadius: borderRadius.sm,
            fontSize: fontSize.xs,
            // ...stylex.include({ ...padding["x-1"], ...padding["y-0.5"] }),
            // text-foreground-500   font-sans group-hover:border-current
        },
    }),
    conditions: {},
    variants: {
        size: {
            sm: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-1.5"], ...padding["y-1"] }),
                    // item: "h-7",
                    // "[&_[data-menu-item]]:h-6 [&_[data-menu-item]_button]:w-6 [&_[data-menu-item]_button]:h-6",
                },
            }),
            md: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-2"], ...padding["y-2"] }),
                    // item: "h-9",
                    // "[&_[data-menu-item]]:h-7 [&_[data-menu-item]_button]:w-7 [&_[data-menu-item]_button]:h-7",
                },
            }),
            lg: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-3"], ...padding["y-3"] }),
                    // item: "h-11",
                    // "[&_[data-menu-item]]:h-8 [&_[data-menu-item]_button]:w-8 [&_[data-menu-item]_button]:h-8",
                },
            }),
        },
        variant: {
            solid: create({
                menuWrapper: {
                    borderRadius: borderRadius.md,
                    // padding: spacing["1"],
                    boxShadow: boxShadow.lg,
                    backgroundColor: colors.backgroundSecondary,
                },
            }),
            light: create({}),
            // light
            // ghost
        },
    },
    defaultVariants: {
        size: "md",
        variant: "solid",
    },
})

export const MenuStyleContext = createNewGenericContext<CachedStyles<typeof menuStyles>>()
