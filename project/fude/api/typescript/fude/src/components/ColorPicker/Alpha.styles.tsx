import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { zIndex } from "@sribich/fude-theme/vars/zindex.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const alphaStyles = makeStyles({
    slots: create({
        alpha: {
            position: "absolute",
            inset: 0,
            zIndex: zIndex.behind1,
            boxShadow: boxShadow.inset,
            borderRadius: "var(--track-radius)",
            overflow: "hidden",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
