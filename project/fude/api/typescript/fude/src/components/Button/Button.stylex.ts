import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create, keyframes } from "@stylexjs/stylex"

import { stylexColorVariants } from "../../theme/atomics/color.js"
import { makeStyles } from "../../theme/props.js"

const spin = keyframes({
    to: {
        transform: "rotate(360deg)",
    },
})

export const buttonStyles = makeStyles({
    slots: create({
        button: {
            position: "relative",
            boxSizing: "border-box",
            border: "none",
            zIndex: 0,
            display: "inline-flex",
            userSelect: "none",
            appearance: "none",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
            whiteSpace: "nowrap",
            outline: "none",
            ":disabled": {
                pointerEvents: "none",
                opacity: 0.5,
            },
            ":is([data-pressed])": {
                transform: "scale(0.97)",
            },
        },
        spin: {
            animation: "spin 1.5s linear infinite",
            animationName: spin,
        },
    }),
    conditions: {
        inGroup: create({}),
        iconOnly: create({}),
    },
    variants: {
        variant: {
            solid: create({}),
            ghost: create({
                button: {
                    borderStyle: "solid",
                    borderWidth: borderWidth.md,
                    backgroundColor: "transparent",
                },
            }),
            light: create({
                button: {
                    border: 0,
                    backgroundColor: "transparent",
                },
            }),
        },
        color: {
            default: create({}),
            primary: create({}),
            secondary: create({}),
            success: create({}),
            warning: create({}),
            danger: create({}),
        },
        size: {
            xs: create({
                button: {
                    gap: newSpacing["4"],
                    fontSize: fontSize["xs"],
                    paddingLeft: newSpacing["4"],
                    paddingRight: newSpacing["4"],
                },
                spin: {
                    height: newSpacing["10"],
                    width: newSpacing["10"],
                },
            }),
            sm: create({
                button: {
                    gap: newSpacing["6"],
                    fontSize: fontSize["xs"],
                    paddingLeft: newSpacing["8"],
                    paddingRight: newSpacing["8"],
                },
                spin: {
                    height: newSpacing["16"],
                    width: newSpacing["16"],
                },
            }),
            md: create({
                button: {
                    gap: newSpacing["8"],
                    fontSize: fontSize["sm"],
                    paddingLeft: newSpacing["12"],
                    paddingRight: newSpacing["12"],
                },
                spin: {
                    height: newSpacing["20"],
                    width: newSpacing["20"],
                },
            }),
            lg: create({
                button: {
                    gap: newSpacing["10"],
                    fontSize: fontSize["md"],
                    paddingLeft: newSpacing["16"],
                    paddingRight: newSpacing["16"],
                },
                spin: {
                    height: newSpacing["24"],
                    width: newSpacing["24"],
                },
            }),
        },
        radius: {
            none: create({
                button: {
                    borderRadius: borderRadius.none,
                },
            }),
            sm: create({
                button: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                button: {
                    borderRadius: borderRadius.md,
                },
            }),
            lg: create({
                button: {
                    borderRadius: borderRadius.lg,
                },
            }),
            full: create({
                button: {
                    borderRadius: borderRadius.full,
                },
            }),
        },
        fullWidth: {
            true: create({
                button: {
                    width: "100%",
                    textAlign: "center",
                },
            }),
            false: create({}),
        },
    },
    defaultVariants: {
        fullWidth: false,
        variant: "solid",
        color: "default",
        size: "md",
        radius: "sm",
    },
    compounds: [
        {
            variants: {
                variant: "solid",
            },
            modify: {
                color: {
                    default: stylexColorVariants.solid.default,
                    primary: stylexColorVariants.solid.primary,
                    secondary: stylexColorVariants.solid.secondary,
                    success: stylexColorVariants.solid.success,
                    warning: stylexColorVariants.solid.warning,
                    danger: stylexColorVariants.solid.danger,
                },
            },
        },
        {
            variants: {
                variant: "ghost",
            },
            modify: {
                color: {
                    default: stylexColorVariants.ghost.default,
                    primary: stylexColorVariants.ghost.primary,
                    secondary: stylexColorVariants.ghost.secondary,
                    success: stylexColorVariants.ghost.success,
                    warning: stylexColorVariants.ghost.warning,
                    danger: stylexColorVariants.ghost.danger,
                },
            },
        },
        {
            variants: {
                variant: "light",
            },
            modify: {
                color: {
                    default: stylexColorVariants.light.default,
                    primary: stylexColorVariants.light.primary,
                    secondary: stylexColorVariants.light.secondary,
                    success: stylexColorVariants.light.success,
                    warning: stylexColorVariants.light.warning,
                    danger: stylexColorVariants.light.danger,
                },
            },
        },
        {
            conditions: {
                inGroup: true,
            },
            modify: {
                variant: {
                    ghost: create({
                        button: {
                            marginInlineStart: `calc(-1 * ${borderWidth.md})`,
                        },
                    }),
                },
                radius: {
                    sm: create({
                        button: {
                            borderRadius: {
                                default: borderRadius.none,
                            },
                            borderStartStartRadius: {
                                ":first-child": borderRadius.sm,
                            },
                            borderEndStartRadius: {
                                ":first-child": borderRadius.sm,
                            },
                            borderStartEndRadius: {
                                ":last-child": borderRadius.sm,
                            },
                            borderEndEndRadius: {
                                ":last-child": borderRadius.sm,
                            },
                        },
                    }),
                    md: create({
                        button: {
                            borderRadius: {
                                default: borderRadius.none,
                            },
                            borderStartStartRadius: {
                                ":first-child": borderRadius.md,
                            },
                            borderEndStartRadius: {
                                ":first-child": borderRadius.md,
                            },
                            borderStartEndRadius: {
                                ":last-child": borderRadius.md,
                            },
                            borderEndEndRadius: {
                                ":last-child": borderRadius.md,
                            },
                        },
                    }),
                    lg: create({
                        button: {
                            borderRadius: {
                                default: borderRadius.none,
                            },
                            borderStartStartRadius: {
                                ":first-child": borderRadius.lg,
                            },
                            borderEndStartRadius: {
                                ":first-child": borderRadius.lg,
                            },
                            borderStartEndRadius: {
                                ":last-child": borderRadius.lg,
                            },
                            borderEndEndRadius: {
                                ":last-child": borderRadius.lg,
                            },
                        },
                    }),
                    full: create({
                        button: {
                            borderRadius: {
                                default: borderRadius.none,
                            },
                            borderStartStartRadius: {
                                ":first-child": borderRadius.full,
                            },
                            borderEndStartRadius: {
                                ":first-child": borderRadius.full,
                            },
                            borderStartEndRadius: {
                                ":last-child": borderRadius.full,
                            },
                            borderEndEndRadius: {
                                ":last-child": borderRadius.full,
                            },
                        },
                    }),
                },
            },
        },
        {
            conditions: {
                iconOnly: true,
            },
            modify: {
                size: {
                    xs: create({
                        button: {
                            height: newSpacing["24"],
                            width: newSpacing["24"],
                        },
                    }),
                    sm: create({
                        button: {
                            height: newSpacing["32"],
                            width: newSpacing["32"],
                        },
                    }),
                    md: create({
                        button: {
                            height: newSpacing["40"],
                            width: newSpacing["40"],
                        },
                    }),
                    lg: create({
                        button: {
                            height: newSpacing["48"],
                            width: newSpacing["48"],
                        },
                    }),
                },
            },
        },
    ],
})

export const buttonGroupStyles = makeStyles({
    slots: create({
        container: {
            display: "inline-flex",
            alignItems: "center",
            justifyContent: "center",
        },
    }),
    conditions: {
        fullWidth: create({}),
    },
    variants: {},
    defaultVariants: {},
})
