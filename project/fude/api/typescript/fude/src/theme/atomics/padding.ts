import { create } from "@stylexjs/stylex"

import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"

export const padding = create({
    "x-1": {
        paddingLeft: spacing["1"],
        paddingRight: spacing["1"],
    },
    "x-1.5": {
        paddingLeft: spacing["1.5"],
        paddingRight: spacing["1.5"],
    },
    "x-2": {
        paddingLeft: spacing["2"],
        paddingRight: spacing["2"],
    },
    "x-3": {
        paddingLeft: spacing["3"],
        paddingRight: spacing["3"],
    },
    "y-0.5": {
        paddingTop: spacing["0.5"],
        paddingBottom: spacing["0.5"],
    },
    "y-1": {
        paddingTop: spacing["1"],
        paddingBottom: spacing["1"],
    },
    "y-2": {
        paddingTop: spacing["2"],
        paddingBottom: spacing["2"],
    },
    "y-3": {
        paddingTop: spacing["3"],
        paddingBottom: spacing["3"],
    },
})
