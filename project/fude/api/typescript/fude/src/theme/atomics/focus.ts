import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { create } from "@stylexjs/stylex"

export const { focusStyles } = create({
    focusStyles: {
        ":is([data-focus-visible])": {
            outline: borderWidth.md,
            outlineStyle: "solid",
            outlineColor: colors.focus,
            outlineOffset: 2,
        },
    },
})

/*
export const hocusClasses = [
    "outline-none",
    "data-[focus-visible=true]:outline-2",
    "data-[focus-visible=true]:outline-focus",
    "data-[focus-visible=true]:outline-offset-2",
]
*/
