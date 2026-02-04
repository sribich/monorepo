import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { absoluteComponentSize } from "@sribich/fude-theme/vars/sizing.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"

export const selectStyles = makeStyles({
    slots: create({
        wrapper: {
            display: "inline-flex",
            flexDirection: "column",
            position: "relative",
            // minWidth: newSpacing["160"],
        },
        triggerContainer: {
            display: "flex",
            flexDirection: "column",
        },
        trigger: {},

        triggerIndicator: {
            position: "absolute",
            insetInlineEnd: newSpacing["12"],
            height: newSpacing["16"],
            width: newSpacing["16"],
        },
        select: {
            // display: "flex",
        },
        label: {
            display: "block",
            position: "absolute",

            alignItems: "center",
            marginRight: newSpacing["4"],

            transformOrigin: "top left",

            transitionProperty: "all",
            transitionDuration: "150ms",
            transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        value: {},
    }),
    variants: {
        labelPlacement: {
            inside: create({}),
            outside: create({
                wrapper: {
                    flexDirection: "column",
                },
            }),
            left: create({
                wrapper: {
                    flexDirection: "row",
                    alignItems: "center",
                },
            }),
        },
        size: {
            sm: create({
                label: {
                    fontSize: fontSize.xs,
                },
                trigger: {
                    height: absoluteComponentSize.sm,
                    minHeight: absoluteComponentSize.sm,
                },
            }),
            md: create({
                label: {
                    fontSize: fontSize.sm,
                },
                trigger: {
                    height: absoluteComponentSize.md,
                    minHeight: absoluteComponentSize.md,
                },
            }),
            lg: create({
                label: {
                    fontSize: fontSize.md,
                },
                trigger: {
                    height: absoluteComponentSize.lg,
                    minHeight: absoluteComponentSize.lg,
                },
            }),
        },
        variant: {
            ghost: create({}),
            light: create({}),
            solid: create({}),
        },
    },
    defaultVariants: {
        labelPlacement: "inside",
        size: "md",
        variant: "solid",
    },
    modifiers: {},
    conditions: {
        filled: {
            true: {},
            false: {},
        },
        open: {
            true: create({
                triggerIndicator: {
                    transform: "rotate(180deg)",
                },
            }),
            false: create({}),
        },
    },
    compounds: [
        {
            variants: {
                labelPlacement: "inside",
            },
            modify: {
                size: {
                    sm: create({
                        trigger: {
                            height: absoluteComponentSize.smInside,
                        },
                    }),
                    md: create({
                        trigger: {
                            height: absoluteComponentSize.mdInside,
                        },
                    }),
                    lg: create({
                        trigger: {
                            height: absoluteComponentSize.lgInside,
                        },
                    }),
                },
            },
        },
        {
            variants: {
                labelPlacement: "inside",
            },
            conditions: {
                filled: true,
            },
            modify: {
                size: {
                    sm: create({
                        label: {
                            transform: `translateY(calc(-1 * 50% + ${fontSize.xs} / 2 - 2px))`,
                        },
                        value: {
                            paddingTop: newSpacing["16"],
                        },
                    }),
                    md: create({
                        label: {
                            transform: `translateY(calc(-1 * 50% + ${fontSize.sm} / 2 - 4px))`,
                        },
                        value: {
                            paddingTop: newSpacing["16"],
                        },
                    }),
                    lg: create({
                        label: {
                            transform: `translateY(calc(-1 * 50% + ${fontSize.md} / 2 - 6px))`,
                        },
                        value: {
                            paddingTop: newSpacing["16"],
                        },
                    }),
                },
            },
        },
    ],
})

export const SelectStyleContext = createNewGenericContext<CachedStyles<typeof selectStyles>>(false)
