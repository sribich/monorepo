import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { newSpacing, spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { zIndex } from "@sribich/fude-theme/vars/zindex.stylex"
import { create } from "@stylexjs/stylex"

import { createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, type ExportedStyles, makeStyles } from "../../theme/props"

export const cardStyles = makeStyles({
    slots: create({
        card: {
            display: "flex",
            flexDirection: "column",
            position: "relative",
            overflow: "hidden",
            height: "auto",
            color: colors.foreground,

            backgroundColor: colors.background,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: colors.borderUi,
            boxShadow: boxShadow.lg,
            width: "fit-content",
        },
        header: {
            display: "flex",
            padding: newSpacing["12"],
            zIndex: zIndex["infront1"],
            justifyContent: "start",
            alignItems: "center",
            flexShrink: 0,
        },
        menuArea: {
            position: "absolute",
            top: 0,
            right: 0,
            zIndex: zIndex["infront2"],
            padding: newSpacing["12"],
        },
        body: {
            padding: spacing["3"],
            overflow: "hidden",
        },
        footer: {
            display: "flex",
            width: "100%",
            height: "auto",
            padding: newSpacing["12"],
            alignItems: "center",
            overflow: "hidden",
        },
    }),
    conditions: {
        isPressable: create({
            card: {
                cursor: "pointer",
            },
        }),
        blurFooter: create({
            footer: {
                backgroundColor: `oklch(from ${colors.background} l c h / .5)`,
                backdropFilter: "blur(4px)",
            },
        }),

        focused: create({}),
    },
    variants: {
        footerStyle: {
            default: create({}),
            floating: create({}),
            sticky: create({
                footer: {
                    position: "absolute",
                    bottom: 0,
                    zIndex: zIndex["infront2"],
                    backgroundColor: colors.background,
                    opacity: 80,
                },
            }),
        },
        rounded: {
            none: create({
                card: {
                    borderRadius: borderRadius.none,
                },
                body: { borderRadius: borderRadius.none },
            }),
            sm: create({
                card: {
                    borderRadius: borderRadius.sm,
                },
                body: { borderRadius: borderRadius.sm },
            }),
            md: create({
                card: {
                    borderRadius: borderRadius.md,
                },
                body: { borderRadius: borderRadius.md },
            }),
            lg: create({
                card: {
                    borderRadius: borderRadius.lg,
                },
                body: { borderRadius: borderRadius.lg },
            }),
            full: create({
                card: {
                    borderRadius: borderRadius.full,
                },
                body: { borderRadius: borderRadius.full },
            }),
        },
    },
    defaultVariants: {
        footerStyle: "default",
        rounded: "sm",
    },
}) satisfies ExportedStyles

export const CardStyleContext = createNewGenericContext<CachedStyles<typeof cardStyles>>()
