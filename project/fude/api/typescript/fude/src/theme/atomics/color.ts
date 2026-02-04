import { create } from "@stylexjs/stylex"

import { colors } from "@sribich/fude-theme/vars/colors.stylex"

export const stylexColorVariantsNonInteractive = {
    solid: create({
        default: {
            color: colors.foreground,
            backgroundColor: colors.backgroundSecondary,
        },
        primary: {
            color: colors.primaryForeground,
            backgroundColor: colors.primary,
        },
        secondary: {},
        success: {
            color: colors.successForeground,
            backgroundColor: colors.success,
        },
        warning: {
            color: colors.warningForeground,
            backgroundColor: colors.warning,
        },
        danger: {
            color: colors.dangerForeground,
            backgroundColor: colors.danger,
        },
    }),
    ghost: create({
        default: {
            borderColor: colors.background,
            backgroundColor: {
                default: "transparent",
                ":hover": colors.background,
            },
        },
        secondary: {},
        primary: {
            borderColor: colors.primary,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.primary,
                color: colors.primaryForeground,
            },
        },
        success: {
            borderColor: colors.success,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.success,
                color: colors.successForeground,
            },
        },
        warning: {
            borderColor: colors.warning,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.warning,
                color: colors.warningForeground,
            },
        },
        danger: {
            borderColor: colors.danger,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.danger,
                color: colors.dangerForeground,
            },
        },
    }),
    light: create({
        default: {
            ":hover": {
                backgroundColor: colors.backgroundHover,
                color: colors.primaryForeground,
            },
        },
        secondary: {},
        primary: {
            ":hover": {
                backgroundColor: colors.primarySelectedHover,
                color: colors.primarySelectedForeground,
            },
        },
        success: {
            ":hover": {
                backgroundColor: colors.successSelectedHover,
                color: colors.successSelectedForeground,
            },
        },
        warning: {
            ":hover": {
                backgroundColor: colors.warningSelectedHover,
                color: colors.warningSelectedForeground,
            },
        },
        danger: {
            ":hover": {
                backgroundColor: colors.danger,
                color: colors.dangerSelectedForeground,
            },
        },
    }),
} as const

export const stylexColorVariants = {
    solid: create({
        default: {
            color: colors.foreground,
            backgroundColor: colors.backgroundSecondary,
            ":hover": {
                backgroundColor: colors.backgroundSecondaryHover,
            },
        },
        primary: {
            color: colors.primaryForeground,
            backgroundColor: colors.primary,
            ":hover": {
                backgroundColor: colors.primaryHover,
            },
        },
        secondary: {},
        success: {
            color: colors.successForeground,
            backgroundColor: colors.success,
            ":hover": {
                backgroundColor: colors.successHover,
            },
        },
        warning: {
            color: colors.warningForeground,
            backgroundColor: colors.warning,
            ":hover": {
                backgroundColor: colors.warningHover,
            },
        },
        danger: {
            color: colors.dangerForeground,
            backgroundColor: colors.danger,
            ":hover": {
                backgroundColor: colors.dangerHover,
            },
        },
    }),
    ghost: create({
        default: {
            borderColor: colors.background,
            backgroundColor: {
                default: "transparent",
                ":hover": colors.background,
            },
        },
        secondary: {},
        primary: {
            borderColor: colors.primary,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.primary,
                color: colors.primaryForeground,
            },
        },
        success: {
            borderColor: colors.success,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.success,
                color: colors.successForeground,
            },
        },
        warning: {
            borderColor: colors.warning,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.warning,
                color: colors.warningForeground,
            },
        },
        danger: {
            borderColor: colors.danger,
            backgroundColor: "transparent",
            ":hover": {
                backgroundColor: colors.danger,
                color: colors.dangerForeground,
            },
        },
    }),
    light: create({
        default: {
            color: colors.foreground,
            ":hover": {
                backgroundColor: colors.backgroundHover,
            },
        },
        secondary: {},
        primary: {
            color: colors.primary,
            ":hover": {
                backgroundColor: colors.primarySelectedHover,
            },
        },
        success: {
            color: colors.success,
            ":hover": {
                backgroundColor: colors.successSelectedHover,
            },
        },
        warning: {
            color: colors.warning,
            ":hover": {
                backgroundColor: colors.warningSelectedHover,
            },
        },
        danger: {
            color: colors.danger,
            ":hover": {
                backgroundColor: colors.danger,
            },
        },
    }),
} as const

/*


export const colorVariants = {
    solid: {
        default: "bg-default text-default-foreground",
        primary: "bg-primary text-primary-foreground",
        secondary: "bg-secondary text-secondary-foreground",
        success: "bg-success text-success-foreground",
        warning: "bg-warning text-warning-foreground",
        danger: "bg-danger text-danger-foreground",
    },
    ghost: {
        default: "border-default text-default-foreground hover:bg-default",
        primary: "border-primary text-primary hover:bg-primary hover:text-primary-foreground",
        secondary:
            "border-secondary text-secondary hover:bg-secondary hover:text-secondary-foreground",
        success: "border-success text-success hover:bg-success hover:text-success-foreground",
        warning: "border-warning text-warning hover:bg-warning hover:text-warning-foreground",
        danger: "border-danger text-danger hover:text-danger-foreground hover:bg-danger hover:text-danger-foreground",
    },
    light: {
        default: "bg-transparent text-default-foreground",
        primary: "bg-transparent text-primary",
        secondary: "bg-transparent text-secondary",
        success: "bg-transparent text-success",
        warning: "bg-transparent text-warning",
        danger: "bg-transparent text-danger",
        foreground: "bg-transparent text-foreground",
    },
}

export const solidVariants = [
    {
        variant: "solid",
        color: "default",
        class: colorVariants.solid.default,
    },
    {
        variant: "solid",
        color: "primary",
        class: colorVariants.solid.primary,
    },
    {
        variant: "solid",
        color: "secondary",
        class: colorVariants.solid.secondary,
    },
    {
        variant: "solid",
        color: "success",
        class: colorVariants.solid.success,
    },
    {
        variant: "solid",
        color: "warning",
        class: colorVariants.solid.warning,
    },
    {
        variant: "solid",
        color: "danger",
        class: colorVariants.solid.danger,
    },
] as const

export const ghostVariants = [
    {
        variant: "ghost",
        color: "default",
        class: colorVariants.ghost.default,
    },
    {
        variant: "ghost",
        color: "primary",
        class: colorVariants.ghost.primary,
    },
    {
        variant: "ghost",
        color: "secondary",
        class: colorVariants.ghost.secondary,
    },
    {
        variant: "ghost",
        color: "success",
        class: colorVariants.ghost.success,
    },
    {
        variant: "ghost",
        color: "warning",
        class: colorVariants.ghost.warning,
    },
    {
        variant: "ghost",
        color: "danger",
        class: colorVariants.ghost.danger,
    },
] as const

export const shadowVariants = {}

export const borderedVariants = {}

export const flatVariants = {}

export const fadedVariants = {}

export const lightVariants = [
    {
        variant: "light",
        color: "default",
        class: `${colorVariants.light.default} data-[hovered=true]:bg-default/40`,
    },
    {
        variant: "light",
        color: "primary",
        class: `${colorVariants.light.primary} data-[hovered=true]:bg-primary/20`,
    },
    {
        variant: "light",
        color: "secondary",
        class: `${colorVariants.light.secondary} data-[hovered=true]:bg-secondary/20`,
    },
    {
        variant: "light",
        color: "success",
        class: `${colorVariants.light.success} data-[hovered=true]:bg-success/20`,
    },
    {
        variant: "light",
        color: "warning",
        class: `${colorVariants.light.warning} data-[hovered=true]:bg-warning/20`,
    },
    {
        variant: "light",
        color: "danger",
        class: `${colorVariants.light.danger} data-[hovered=true]:bg-danger/20`,
    },
] as const
*/
