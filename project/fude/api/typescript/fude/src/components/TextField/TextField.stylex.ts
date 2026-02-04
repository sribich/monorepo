import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const textFieldStyles = makeStyles({
    slots: create({
        textField: {
            display: "flex",
            flexDirection: "column",
            width: "100%",
        },
        inputGroup: {
            position: "relative",
            width: "100%",
            display: "inline-flex",
            flexDirection: "row",
            alignItems: "center",
            paddingInline: newSpacing["12"],
        },
        inputInner: {
            display: "inline-flex",
            height: "100%",
            width: "100%",
            alignItems: "center",
            boxSizing: "border-box",
        },
        input: {
            willChange: "auto",
            width: "100%",
            backgroundColor: "transparent",
            outlineStyle: "solid",
            outlineColor: "transparent",
            border: "none",
            // [when.ancestor(fieldMarker)]
        },
        label: {
            position: "absolute",
            pointerEvents: "none",
            display: "block",
            color: colors.secondaryForeground,
            transformOrigin: "top-left",
        },
    }),
    conditions: {
        focused: {},
        hasContent: {},
    },
    variants: {
        variant: {
            flat: create({
                inputGroup: {
                    backgroundColor: {
                        default: colors.backgroundSecondary,
                        ":hover": colors.backgroundSecondaryHover,
                    },
                },
            }),
            faded: create({
                inputGroup: {
                    backgroundColor: colors.backgroundSecondary,
                    borderWidth: borderWidth.lg,
                    borderColor: {
                        default: colors.borderUi,
                        ":hover": colors.borderUiHover,
                    },
                },
            }),
            bordered: create({
                inputGroup: {
                    borderWidth: borderWidth.lg,
                    borderColor: {
                        default: colors.borderUi,
                        ":hover": colors.borderUiHover,
                    },
                },
            }),
            underlined: create({
                inputGroup: {},
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
            sm: create({
                textField: {
                    "--label-font-size": fontSize.xs,
                },
                label: {
                    fontSize: fontSize.xs,
                },
                inputGroup: {
                    height: newSpacing["32"],
                },
                input: {
                    fontSize: fontSize.sm,
                },
            }),
            md: create({
                textField: {
                    "--label-font-size": fontSize.sm,
                },
                label: {
                    fontSize: fontSize.sm,
                },
                inputGroup: {
                    height: newSpacing["40"],
                },
                input: {
                    fontSize: fontSize.sm,
                },
            }),
            lg: create({
                textField: {
                    "--label-font-size": fontSize.md,
                },
                label: {
                    fontSize: fontSize.md,
                },
                inputGroup: {
                    height: newSpacing["48"],
                },
                input: {
                    fontSize: fontSize.md,
                },
            }),
        },
        radius: {
            none: create({
                inputGroup: {
                    borderRadius: borderRadius.none,
                },
            }),
            sm: create({
                inputGroup: {
                    borderRadius: borderRadius.sm,
                },
            }),
            md: create({
                inputGroup: {
                    borderRadius: borderRadius.md,
                },
            }),
            lg: create({
                inputGroup: {
                    borderRadius: borderRadius.lg,
                },
            }),
            full: create({
                inputGroup: {
                    borderRadius: borderRadius.full,
                },
            }),
        },
        labelPlacement: {
            outside: create({
                textField: {
                    display: "flex",
                    flexDirection: "column",
                },
            }),
            "outside-left": create({}),
            "outside-top": create({}),
            inside: create({
                label: {
                    cursor: "text",
                },
                inputGroup: {
                    flexDirection: "column",
                    alignItems: "start",
                    justifyContent: "center",
                    gap: 0,
                },
                inputInner: {
                    alignItems: "end",
                },
            }),
        },
    },
    defaultVariants: {
        variant: "flat",
        color: "default",
        size: "md",
        radius: "md",
        labelPlacement: "inside",
    },
    compounds: [
        // Add padding to fully rounded components to prevent overflow.
        {
            variants: {
                radius: "full",
            },
            modify: {
                size: {
                    sm: create({
                        inputGroup: { paddingInline: newSpacing["12"] },
                    }),
                    md: create({
                        inputGroup: { paddingInline: newSpacing["16"] },
                    }),
                    lg: create({
                        inputGroup: { paddingInline: newSpacing["20"] },
                    }),
                },
            },
        },
        {
            variants: {
                labelPlacement: "inside",
            },
            modify: {
                size: {
                    sm: create({
                        inputGroup: {
                            height: newSpacing["48"],
                            paddingBlock: newSpacing["6"],
                            paddingInline: newSpacing["12"],
                        },
                        label: {
                            fontSize: fontSize.sm,
                        },
                    }),
                    md: create({
                        inputGroup: {
                            height: newSpacing["56"],
                            paddingBlock: newSpacing["8"],
                        },
                        label: {
                            fontSize: fontSize.sm,
                        },
                    }),
                    lg: create({
                        inputGroup: {
                            height: newSpacing["64"],
                            paddingBlock: newSpacing["10"],
                        },
                        label: {
                            fontSize: fontSize.md,
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
                hasContent: true,
            },
            modify: {
                size: {
                    sm: create({
                        label: {
                            transform:
                                "translateY(calc(-1 * 50% + var(--label-font-size) / 2 - 8px))",
                        },
                    }),
                    md: create({
                        label: {
                            transform:
                                "translateY(calc(-1 * 50% + var(--label-font-size) / 2 - 8px))",
                        },
                    }),
                    lg: create({
                        label: {
                            transform:
                                "translateY(calc(-1 * 50% + var(--label-font-size) / 2 - 8px))",
                        },
                    }),
                },
            },
        },
        {
            conditions: {
                focused: true,
            },
            modify: {
                variant: {
                    faded: create({
                        inputGroup: {
                            borderColor: colors.borderUiHover,
                        },
                    }),
                    bordered: create({
                        inputGroup: {
                            borderColor: colors.borderUiSelected,
                        },
                    }),
                },
            },
        },
    ],
})
