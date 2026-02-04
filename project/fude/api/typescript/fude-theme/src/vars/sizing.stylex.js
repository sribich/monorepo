import { defineVars } from "@stylexjs/stylex"

import { spacing } from "./spacing.stylex"

export const absoluteComponentSize = defineVars({
    xs: spacing["6"],
    sm: spacing["8"],
    md: spacing["10"],
    lg: spacing["12"],

    smInside: spacing["12"],
    mdInside: spacing["14"],
    lgInside: spacing["16"],
})
