import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { lineHeight } from "@sribich/fude-theme/vars/lineHeight.stylex"
import { absoluteComponentSize } from "@sribich/fude-theme/vars/sizing.stylex"
import { create } from "@stylexjs/stylex"

export const componentSize = create({
    xs: {
        height: absoluteComponentSize.xs,
        fontSize: fontSize.sm,
        lineHeight: lineHeight.xs,
    },
    sm: {
        height: absoluteComponentSize.sm,
        fontSize: fontSize.sm,
        lineHeight: lineHeight.sm,
    },
    md: {
        height: absoluteComponentSize.md,
        fontSize: fontSize.md,
        lineHeight: lineHeight.md,
    },
    lg: {
        height: absoluteComponentSize.lg,
        fontSize: fontSize.lg,
        lineHeight: lineHeight.lg,
    },
})
